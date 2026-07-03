use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;
use tauri::Manager;
use futures_util::StreamExt;
use std::sync::OnceLock;
use tiktoken_rs::CoreBPE;

use crate::cancel::StreamCancelState;

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
static BPE: OnceLock<CoreBPE> = OnceLock::new();

fn get_bpe() -> &'static CoreBPE {
    BPE.get_or_init(|| tiktoken_rs::cl100k_base().expect("Failed to load cl100k_base tokenizer"))
}

pub(crate) fn get_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .pool_max_idle_per_host(2)
            .timeout(std::time::Duration::from_secs(120))
            .read_timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client")
    })
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatRequest {
    pub provider: String,
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub system_prompt: Option<String>,
    pub active_workspace: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct StreamDelta {
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: Option<StreamDelta>,
    #[allow(dead_code)]
    index: u32,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StreamResponse {
    choices: Vec<StreamChoice>,
}

fn provider_endpoint(provider: &str) -> &'static str {
    match provider {
        "deepseek" => "https://api.deepseek.com/v1/chat/completions",
        "opencode_go" => "https://opencode.ai/zen/go/v1/chat/completions",
        "opencode_zen" => "https://opencode.ai/zen/v1/chat/completions",
        _ => "https://api.deepseek.com/v1/chat/completions",
    }
}

#[tauri::command]
pub fn abort_stream(state: tauri::State<'_, StreamCancelState>) {
    state.cancel();
}

/// Tauri command to stream a chat completion from an LLM provider over a Tauri IPC channel.
/// Sends content deltas, token counts, and stream lifecycle events to the frontend.
#[tauri::command]
pub async fn stream_chat(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, sqlx::SqlitePool>,
    cancel_state: tauri::State<'_, StreamCancelState>,
    channel: Channel<String>,
    request: ChatRequest,
) -> Result<(), String> {
    let api_key = get_api_key(&app_handle, &state, &request.provider).await?;
    let endpoint = provider_endpoint(&request.provider);
    let client = get_client();

    let mut messages: Vec<ChatMessage> = Vec::new();

    let system_content = request.system_prompt.clone()
        .unwrap_or_else(|| "You are Clutch, a helpful desktop AI assistant. You respond to questions directly — do not generate tool calls, commands, or function syntax.\n\nThe user can:\n- Share file contents by pasting them directly or adding folders in Settings → Workspaces, then asking you about them\n- Run commands themselves in their terminal\n- Install skills from skills.sh\n\nAlways respond in the same language the user uses. Be concise and direct.".to_string());

    messages.push(ChatMessage {
        role: "system".to_string(),
        content: system_content,
    });

    messages.extend(request.messages.clone());

    if let Some(ref msg) = messages.iter().rev().find(|m| m.role == "user") {
        let enhanced = crate::file_resolver::inject_file_contents(
            &msg.content,
            request.active_workspace.as_deref(),
        ).await;
        if let Some(last_user) = messages.iter_mut().rev().find(|m| m.role == "user") {
            last_user.content = enhanced;
        }
    }

    let body = serde_json::json!({
        "model": request.model,
        "messages": messages.iter().map(|m| {
            serde_json::json!({
                "role": m.role,
                "content": m.content,
            })
        }).collect::<Vec<_>>(),
        "stream": true,
        "temperature": request.temperature.unwrap_or(0.7),
        "max_tokens": request.max_tokens.unwrap_or(4096),
    });

    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("Accept", "text/event-stream")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        let msg = serde_json::from_str::<serde_json::Value>(&body)
            .ok()
            .and_then(|v| v["error"]["message"].as_str().map(String::from))
            .unwrap_or(body);
        return Err(format!("API error {}: {}", status, msg));
    }

    let mut stream = response.bytes_stream();
    let mut buffer: Vec<u8> = Vec::new();
    let mut content_received = false;
    let mut tokens_received: u64 = 0;
    let mut cancel_rx = cancel_state.reset_and_subscribe();

    loop {
        tokio::select! {
            chunk_result = stream.next() => {
                let chunk = match chunk_result {
                    Some(c) => match c {
                        Ok(c) => c,
                        Err(e) => {
                            drain_remaining_buffer(&buffer, &mut content_received, &mut tokens_received, &channel)?;
                            if content_received {
                                channel.send("__STREAM_INTERRUPTED__".to_string())
                                    .map_err(|e| format!("Channel send error: {}", e))?;
                                return Ok(());
                            }
                            return Err(format!("Stream error: {}", e));
                        },
                    },
                    None => break,
                };

        buffer.extend_from_slice(&chunk);

        while let Some(pos) = buffer.iter().position(|&b| b == b'\n') {
            let line = std::str::from_utf8(&buffer[..pos])
                .map_err(|_| "Invalid UTF-8 in stream".to_string())?
                .trim()
                .to_string();
            buffer.drain(..=pos);

            if line.is_empty() || line.starts_with(':') {
                continue;
            }

            if let Some(data) = line.strip_prefix("data: ") {
                if data == "[DONE]" {
                    drain_remaining_buffer(&buffer, &mut content_received, &mut tokens_received, &channel)?;
                    if content_received {
                        channel.send(format!("__TOKENS__:{}", tokens_received))
                            .map_err(|e| format!("Channel send error: {}", e))?;
                    } else {
                        return Err("Provider returned empty response".to_string());
                    }
                    return Ok(());
                }

                if let Ok(response) = serde_json::from_str::<StreamResponse>(data) {
                    if let Some(choice) = response.choices.first() {
                        if let Some(delta) = &choice.delta {
                            if let Some(content) = &delta.content {
                                content_received = true;
                                let count = get_bpe().encode_with_special_tokens(content).len() as u64;
                                tokens_received += count;
                                channel.send(content.clone())
                                    .map_err(|e| format!("Channel send error: {}", e))?;
                                channel.send(format!("__TOKENS_LIVE__:{}", tokens_received))
                                    .map_err(|e| format!("Channel send error: {}", e))?;
                            }
                        }

                        if let Some(finish_reason) = &choice.finish_reason {
                            if finish_reason == "content_filter" {
                                return Err("Response blocked by content filter".to_string());
                            }
                            drain_remaining_buffer(&buffer, &mut content_received, &mut tokens_received, &channel)?;
                            if content_received {
                                channel.send(format!("__TOKENS__:{}", tokens_received))
                                    .map_err(|e| format!("Channel send error: {}", e))?;
                            } else {
                                return Err("Provider returned empty response".to_string());
                            }
                            return Ok(());
                        }
                    }
                }
            }
        }
            }
            _ = cancel_rx.changed() => {
                if *cancel_rx.borrow() {
                    drain_remaining_buffer(&buffer, &mut content_received, &mut tokens_received, &channel)?;
                    if content_received {
                        channel.send("__STREAM_INTERRUPTED__".to_string())
                            .map_err(|e| format!("Channel send error: {}", e))?;
                    }
                    return Ok(());
                }
            }
        }
    }

    if content_received {
        channel.send(format!("__TOKENS__:{}", tokens_received))
            .map_err(|e| format!("Channel send error: {}", e))?;
    } else {
        return Err("Stream ended without any content".to_string());
    }
    Ok(())
}

fn drain_remaining_buffer(
    buffer: &[u8],
    content_received: &mut bool,
    tokens_received: &mut u64,
    channel: &Channel<String>,
) -> Result<(), String> {
    let text = std::str::from_utf8(buffer)
        .map_err(|_| "Invalid UTF-8 in stream tail".to_string())?;
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    if let Some(data) = trimmed.strip_prefix("data: ") {
        if data == "[DONE]" {
            return Ok(());
        }
        if let Ok(response) = serde_json::from_str::<StreamResponse>(data) {
            if let Some(choice) = response.choices.first() {
                if let Some(delta) = &choice.delta {
                    if let Some(content) = &delta.content {
                        *content_received = true;
                        let count = get_bpe().encode_with_special_tokens(content).len() as u64;
                        *tokens_received += count;
                        channel.send(content.clone()).map_err(|e| format!("Channel send error: {}", e))?;
                    }
                }
            }
        }
    }

    Ok(())
}

async fn get_api_key(
    app_handle: &tauri::AppHandle,
    pool: &sqlx::SqlitePool,
    provider: &str,
) -> Result<String, String> {
    use sha2::{Digest, Sha256};

    let key = format!("api_key_{}", provider);
    let row: (String,) = sqlx::query_as("SELECT value FROM settings WHERE key = ?")
        .bind(&key)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?
        .ok_or_else(|| format!("API key not found for provider: {}", provider))?;

    let data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    let path_str = data_dir.to_string_lossy();
    let mut hasher = Sha256::new();
    hasher.update(b"clutch-api-key-salt-v1");
    hasher.update(path_str.as_bytes());
    let hash = hasher.finalize();
    let mut enc_key = [0u8; 32];
    enc_key.copy_from_slice(&hash);

    use aes_gcm::aead::Aead;
    use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
    use base64::engine::general_purpose::STANDARD as BASE64;
    use base64::Engine;

    let raw = &row.0;
    if let Some((nonce_b64, ct_b64)) = raw.split_once(':') {
        let nonce_bytes = BASE64
            .decode(nonce_b64)
            .map_err(|e| format!("Failed to decode nonce: {}", e))?;
        let ct_bytes = BASE64
            .decode(ct_b64)
            .map_err(|e| format!("Failed to decode ciphertext: {}", e))?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let key = Key::<Aes256Gcm>::from_slice(&enc_key);
        let cipher = Aes256Gcm::new(key);
        let plaintext = cipher
            .decrypt(nonce, ct_bytes.as_ref())
            .map_err(|e| format!("Decryption failed: {}", e))?;
        String::from_utf8(plaintext).map_err(|e| format!("Invalid UTF-8: {}", e))
    } else {
        Ok(raw.to_string())
    }
}
