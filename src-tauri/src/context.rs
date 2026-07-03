use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCount {
    pub tokens: usize,
    pub model: String,
}

fn model_encoding(model: &str) -> &str {
    if model.starts_with("gpt-") || model.starts_with("o1-") || model.starts_with("o3-") {
        "o200k_base"
    } else if model.starts_with("claude-") {
        "cl100k_base"
    } else {
        "cl100k_base"
    }
}

/// Returns the context window size limit in tokens for the given model identifier string.
pub fn get_model_limit(model: &str) -> u32 {
    if model.contains("deepseek-") { 1048576 }
    else if model.contains("gemini-") { 1048576 }
    else if model.contains("qwen") { 262144 }
    else if model.contains("gpt-5.4") || model.contains("gpt-5.5") || model.contains("gpt-5.3") { 272000 }
    else if model.contains("gpt-") { 128000 }
    else if model.contains("claude-") { 200000 }
    else if model.contains("minimax-") || model.contains("mimo-") { 262144 }
    else if model.contains("grok-") { 131072 }
    else if model.contains("kimi-") || model.contains("glm-") { 128000 }
    else { 8192 }
}

/// Counts the number of tokens in the given text using the appropriate BPE tokenizer for the model.
pub fn count_tokens(text: &str, model: &str) -> Result<usize, String> {
    let bpe = match model_encoding(model) {
        "o200k_base" => tiktoken_rs::o200k_base()
            .map_err(|e| format!("Failed to get o200k_base encoder: {}", e))?,
        _ => tiktoken_rs::cl100k_base()
            .map_err(|e| format!("Failed to get cl100k_base encoder: {}", e))?,
    };
    Ok(bpe.encode_with_special_tokens(text).len())
}

/// Trims a list of messages to fit within a given token limit, keeping the most recent messages.
/// Preserves the system prompt and removes older messages first when the limit is exceeded.
pub fn auto_trim(
    messages: &[(String, String)],
    system_prompt: Option<&str>,
    context_content: Option<&str>,
    model_limit: u32,
    keep_last: usize,
) -> Vec<(String, String)> {
    let limit = (model_limit as f64 * 0.9) as usize;
    let mut trimmed = Vec::new();
    let mut token_total: usize = 0;

    if let Some(sp) = system_prompt {
        let sp_tokens = count_tokens(sp, "default").unwrap_or_else(|e| {
            tracing::warn!("Token count failed for system prompt: {}", e);
            sp.len() / 4
        });
        token_total += sp_tokens;
    }

    if let Some(cc) = context_content {
        let cc_tokens = count_tokens(cc, "default").unwrap_or_else(|e| {
            tracing::warn!("Token count failed for context content: {}", e);
            cc.len() / 4
        });
        token_total += cc_tokens;
    }

    for (role, content) in messages.iter().rev().take(keep_last) {
        let msg_tokens = count_tokens(content, "default").unwrap_or_else(|e| {
            tracing::warn!("Token count failed for message: {}", e);
            content.len() / 4
        });
        if token_total + msg_tokens > limit {
            break;
        }
        token_total += msg_tokens;
        trimmed.push((role.clone(), content.clone()));
    }

    trimmed.reverse();
    trimmed
}

/// Tauri command to count tokens in a message string for a given model.
#[tauri::command]
pub fn count_message_tokens(text: String, model: String) -> Result<TokenCount, String> {
    let tokens = count_tokens(&text, &model)?;
    Ok(TokenCount { tokens, model })
}

/// Tauri command to retrieve the context window size limit for the given model.
#[tauri::command]
pub fn get_context_limit(model: String) -> u32 {
    get_model_limit(&model)
}

/// Tauri command to trim a conversation context to fit within the model's token limit,
/// keeping the system prompt and the most recent messages.
#[tauri::command]
pub fn auto_trim_context(
    messages: Vec<(String, String)>,
    system_prompt: Option<String>,
    context_content: Option<String>,
    model: String,
    keep_last: Option<usize>,
) -> Vec<(String, String)> {
    let limit = get_model_limit(&model);
    let keep = keep_last.unwrap_or(4);
    auto_trim(
        &messages,
        system_prompt.as_deref(),
        context_content.as_deref(),
        limit,
        keep,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_limit_deepseek() {
        assert_eq!(get_model_limit("deepseek-v3"), 1048576);
        assert_eq!(get_model_limit("deepseek-chat"), 1048576);
    }

    #[test]
    fn model_limit_gemini() {
        assert_eq!(get_model_limit("gemini-2.0-flash"), 1048576);
        assert_eq!(get_model_limit("gemini-pro"), 1048576);
    }

    #[test]
    fn model_limit_qwen() {
        assert_eq!(get_model_limit("qwen-2.5"), 262144);
        assert_eq!(get_model_limit("qwen-max"), 262144);
    }

    #[test]
    fn model_limit_gpt_5_x() {
        assert_eq!(get_model_limit("gpt-5.4-mini"), 272000);
        assert_eq!(get_model_limit("gpt-5.5"), 272000);
        assert_eq!(get_model_limit("gpt-5.3-turbo"), 272000);
    }

    #[test]
    fn model_limit_gpt_generic() {
        assert_eq!(get_model_limit("gpt-4o"), 128000);
        assert_eq!(get_model_limit("gpt-3.5-turbo"), 128000);
    }

    #[test]
    fn model_limit_claude() {
        assert_eq!(get_model_limit("claude-3-sonnet"), 200000);
        assert_eq!(get_model_limit("claude-sonnet"), 200000);
    }

    #[test]
    fn model_limit_minimax_mimo() {
        assert_eq!(get_model_limit("minimax-m1"), 262144);
        assert_eq!(get_model_limit("mimo-v1"), 262144);
    }

    #[test]
    fn model_limit_grok() {
        assert_eq!(get_model_limit("grok-2"), 131072);
    }

    #[test]
    fn model_limit_kimi_glm() {
        assert_eq!(get_model_limit("kimi-k2"), 128000);
        assert_eq!(get_model_limit("glm-4"), 128000);
    }

    #[test]
    fn model_limit_unknown() {
        assert_eq!(get_model_limit("unknown-model"), 8192);
        assert_eq!(get_model_limit(""), 8192);
    }

    #[test]
    fn count_tokens_empty() {
        let tokens = count_tokens("", "gpt-4").unwrap();
        assert_eq!(tokens, 0);
    }

    #[test]
    fn count_tokens_simple_ascii() {
        let tokens = count_tokens("Hello world", "gpt-4").unwrap();
        assert!(tokens > 0);
    }

    #[test]
    fn count_tokens_long_text() {
        let short = count_tokens("hi", "gpt-4").unwrap();
        let long = count_tokens(
            &"The quick brown fox jumps over the lazy dog".repeat(20),
            "gpt-4",
        )
        .unwrap();
        assert!(long > short);
    }

    #[test]
    fn count_tokens_claude_model() {
        let tokens = count_tokens("Test message for Claude", "claude-3-sonnet").unwrap();
        assert!(tokens > 0);
    }

    #[test]
    fn auto_trim_under_limit_returns_all_within_keep_last() {
        let msgs: Vec<(String, String)> = vec![
            ("user".into(), "Hi".into()),
            ("assistant".into(), "Hello".into()),
            ("user".into(), "How are you?".into()),
        ];
        let result = auto_trim(&msgs, None, None, 100000, 10);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].0, "user");
        assert_eq!(result[2].0, "user");
    }

    #[test]
    fn auto_trim_keep_last_limits_message_count() {
        let msgs: Vec<(String, String)> = (0..10)
            .map(|i| ("user".into(), format!("msg {}", i)))
            .collect();
        let result = auto_trim(&msgs, None, None, 100000, 3);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].1, "msg 7");
        assert_eq!(result[2].1, "msg 9");
    }

    #[test]
    fn auto_trim_keep_last_zero_removes_all() {
        let msgs: Vec<(String, String)> = vec![
            ("user".into(), "Hello".into()),
            ("assistant".into(), "Hi".into()),
        ];
        let result = auto_trim(&msgs, None, None, 100000, 0);
        assert!(result.is_empty());
    }

    #[test]
    fn auto_trim_system_prompt_reduces_capacity() {
        let msgs: Vec<(String, String)> = vec![
            ("user".into(), "Hello".into()),
            ("assistant".into(), "Hi".into()),
        ];
        let no_system = auto_trim(&msgs, None, None, 100, 10);
        let with_system = auto_trim(&msgs, Some(&"X".repeat(2000)), None, 100, 10);
        assert!(with_system.len() <= no_system.len());
    }

    #[test]
    fn auto_trim_system_prompt_none_works() {
        let msgs: Vec<(String, String)> = vec![
            ("user".into(), "Hello".into()),
            ("assistant".into(), "Hi there!".into()),
        ];
        let result = auto_trim(&msgs, None, None, 100000, 10);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn auto_trim_large_messages_trimmed() {
        let msgs: Vec<(String, String)> = vec![
            ("user".into(), "X".repeat(2000)),
            ("assistant".into(), "X".repeat(2000)),
            ("user".into(), "X".repeat(2000)),
            ("assistant".into(), "X".repeat(2000)),
            ("user".into(), "X".repeat(2000)),
        ];
        let result = auto_trim(&msgs, None, None, 1000, 5);
        assert!(result.len() < 5);
    }

    #[test]
    fn auto_trim_empty_messages() {
        let msgs: Vec<(String, String)> = vec![];
        let result = auto_trim(&msgs, None, None, 100000, 5);
        assert!(result.is_empty());
    }

    #[test]
    fn auto_trim_small_limit_no_panic() {
        let msgs: Vec<(String, String)> = vec![
            ("user".into(), "Hello world".into()),
        ];
        let result = auto_trim(&msgs, None, None, 1, 5);
        assert!(result.is_empty());
    }

    #[test]
    fn auto_trim_small_limit_with_system_prompt_no_panic() {
        let msgs: Vec<(String, String)> = vec![
            ("user".into(), "Hello world".into()),
        ];
        let result = auto_trim(&msgs, Some("You are helpful"), None, 1, 5);
        assert!(result.is_empty());
    }

    #[test]
    fn auto_trim_oldest_trimmed_first() {
        let msgs: Vec<(String, String)> = vec![
            ("user".into(), "X".repeat(5000)),
            ("assistant".into(), "X".repeat(5000)),
            ("user".into(), "recent msg 1".into()),
            ("assistant".into(), "recent msg 2".into()),
        ];
        let result = auto_trim(&msgs, None, None, 50, 4);
        assert_eq!(result.len(), 2);
        assert!(result[0].1.contains("recent"));
        assert!(result[1].1.contains("recent"));
    }
}
