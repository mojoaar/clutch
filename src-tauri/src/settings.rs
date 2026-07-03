use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit, Nonce};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::Row;
use sqlx::SqlitePool;
use std::collections::HashMap;
use tauri::Manager;
use tauri::State;

fn derive_encryption_key<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>) -> Result<[u8; 32], String> {
    let data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    let path_str = data_dir.to_string_lossy();
    let mut hasher = Sha256::new();
    hasher.update(b"clutch-api-key-salt-v1");
    hasher.update(path_str.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    Ok(key)
}

fn encrypt_api_key(value: &str, key_bytes: &[u8; 32]) -> Result<String, String> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, value.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;
    let nonce_b64 = BASE64.encode(&nonce);
    let ct_b64 = BASE64.encode(&ciphertext);
    Ok(format!("{}:{}", nonce_b64, ct_b64))
}

fn decrypt_api_key(encrypted: &str, key_bytes: &[u8; 32]) -> Result<String, String> {
    let (nonce_b64, ct_b64) = encrypted
        .split_once(':')
        .ok_or_else(|| "Invalid encrypted format".to_string())?;
    let nonce_bytes = BASE64
        .decode(nonce_b64)
        .map_err(|e| format!("Failed to decode nonce: {}", e))?;
    let ct_bytes = BASE64
        .decode(ct_b64)
        .map_err(|e| format!("Failed to decode ciphertext: {}", e))?;
    let nonce = Nonce::from_slice(&nonce_bytes);
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let plaintext = cipher
        .decrypt(nonce, ct_bytes.as_ref())
        .map_err(|e| format!("Decryption failed: {}", e))?;
    String::from_utf8(plaintext).map_err(|e| format!("Invalid UTF-8: {}", e))
}

fn is_api_key_setting(key: &str) -> bool {
    key.starts_with("api_key_")
}

/// Tauri command to retrieve a single setting value by key.
/// Automatically decrypts API key values before returning them.
#[tauri::command]
pub async fn get_setting<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    pool: State<'_, SqlitePool>,
    key: String,
) -> Result<Option<String>, String> {
    let row = sqlx::query("SELECT value FROM settings WHERE key = ?")
        .bind(&key)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    let value: Option<String> = row.map(|r: sqlx::sqlite::SqliteRow| r.get("value"));

    if is_api_key_setting(&key) {
        match value {
            Some(v) => {
                let enc_key = derive_encryption_key(&app_handle)?;
                decrypt_api_key(&v, &enc_key)
                    .or_else(|_| Ok(v))
                    .map(Some)
            }
            None => Ok(None),
        }
    } else {
        Ok(value)
    }
}

/// Tauri command to set a single setting value.
/// Automatically encrypts API key values before storing them in the database.
#[tauri::command]
pub async fn set_setting<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    pool: State<'_, SqlitePool>,
    key: String,
    value: String,
) -> Result<(), String> {
    let stored_value = if is_api_key_setting(&key) {
        let enc_key = derive_encryption_key(&app_handle)?;
        encrypt_api_key(&value, &enc_key)?
    } else {
        value
    };

    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO settings (key, value, updated_at) VALUES (?, ?, ?) ON CONFLICT(key) DO UPDATE SET value = ?, updated_at = ?",
    )
    .bind(&key)
    .bind(&stored_value)
    .bind(&now)
    .bind(&stored_value)
    .bind(&now)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Tauri command to retrieve all settings as a key-value map.
/// API keys are excluded from the result for security.
#[tauri::command]
pub async fn get_all_settings(
    pool: State<'_, SqlitePool>,
) -> Result<HashMap<String, String>, String> {
    let rows = sqlx::query("SELECT key, value FROM settings")
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    let mut map = HashMap::new();
    for row in rows {
        let key: String = row.get(0);
        let value: String = row.get(1);
        if is_api_key_setting(&key) {
            continue;
        }
        map.insert(key, value);
    }
    Ok(map)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionResult {
    pub ok: bool,
    pub message: String,
}

/// Tauri command to test connectivity to a provider's API using the given API key.
/// Returns whether the connection was successful and a descriptive message.
#[tauri::command]
pub async fn test_connection(
    provider: String,
    api_key: String,
) -> Result<TestConnectionResult, String> {
    if api_key.trim().is_empty() {
        return Ok(TestConnectionResult {
            ok: false,
            message: "API key is empty".into(),
        });
    }

    let endpoint = match provider.as_str() {
        "deepseek" => "https://api.deepseek.com/v1/models",
        "opencode_go" => "https://opencode.ai/zen/go/v1/models",
        "opencode_zen" => "https://opencode.ai/zen/v1/models",
        _ => {
            return Ok(TestConnectionResult {
                ok: false,
                message: format!("Unknown provider: {}", provider),
            })
        }
    };

    let client = crate::api::get_client();
    match client
        .get(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(resp) => {
            let status = resp.status();
            if status.is_success() {
                Ok(TestConnectionResult {
                    ok: true,
                    message: "Connection successful".into(),
                })
            } else {
                let body = resp.text().await.unwrap_or_default();
                Ok(TestConnectionResult {
                    ok: false,
                    message: format!("{} {}", status.as_u16(), body),
                })
            }
        }
        Err(e) => Ok(TestConnectionResult {
            ok: false,
            message: e.to_string(),
        }),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceEntry {
    pub currency: String,
    pub total_balance: String,
    pub granted_balance: String,
    pub topped_up_balance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceInfo {
    pub is_available: bool,
    pub balance_infos: Vec<BalanceEntry>,
}

/// Tauri command to fetch the account balance from a provider (currently DeepSeek only).
/// Returns balance information if available, or None if the provider does not support it.
#[tauri::command]
pub async fn get_balance<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    pool: State<'_, SqlitePool>,
    provider: String,
) -> Result<Option<BalanceInfo>, String> {
    let key_k = format!("api_key_{}", provider);
    let row = sqlx::query("SELECT value FROM settings WHERE key = ?")
        .bind(&key_k)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    let encrypted: String = match row {
        Some(r) => r.get("value"),
        None => return Ok(None),
    };

    if encrypted.trim().is_empty() {
        return Ok(None);
    }

    let enc_key = derive_encryption_key(&app_handle)?;
    let api_key = decrypt_api_key(&encrypted, &enc_key).unwrap_or(encrypted);

    if api_key.trim().is_empty() {
        return Ok(None);
    }

    let endpoint = match provider.as_str() {
        "deepseek" => "https://api.deepseek.com/user/balance",
        _ => return Ok(None),
    };

    let client = crate::api::get_client();
    match client
        .get(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                let info: BalanceInfo = resp
                    .json()
                    .await
                    .map_err(|e| format!("Failed to parse balance: {}", e))?;
                Ok(Some(info))
            } else {
                Ok(None)
            }
        }
        Err(e) => {
            tracing::warn!("Failed to fetch balance for {}: {}", provider, e);
            Ok(None)
        }
    }
}

/// Tauri command to return the current application version from the package manifest.
#[tauri::command]
pub fn get_app_version<R: tauri::Runtime>(app: tauri::AppHandle<R>) -> String {
    app.package_info().version.to_string()
}

/// Toggles the visibility of the system tray icon.
#[tauri::command]
pub async fn toggle_tray_icon<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    pool: tauri::State<'_, SqlitePool>,
    visible: bool,
) -> Result<(), String> {
    if let Some(tray) = app_handle.tray_by_id("main-tray") {
        let _ = tray.set_visible(visible);
        let _ = tray.set_show_menu_on_left_click(false);
    }
    sqlx::query("INSERT INTO settings (key, value) VALUES ('show_tray_icon', ?1) ON CONFLICT(key) DO UPDATE SET value = ?1, updated_at = datetime('now')")
        .bind(visible.to_string())
        .execute(pool.inner())
        .await
        .map_err(|e| format!("Failed to save tray icon setting: {}", e))?;

    Ok(())
}
