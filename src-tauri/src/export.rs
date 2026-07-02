use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportOptions {
    pub format: String, // "markdown", "text", "json", "html"
    pub include_metadata: bool,
    pub include_timestamps: bool,
    pub include_provider_info: bool,
    pub include_system_prompt: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportMessage {
    pub id: String,
    pub role: String,
    pub content: String,
    pub created_at: Option<String>,
    pub tokens_used: Option<i64>,
    pub is_deleted: Option<bool>,
    pub edited_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportSession {
    pub id: String,
    pub title: String,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub system_prompt: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub token_count: Option<i64>,
}

/// Tauri command to export a chat session in the specified format (markdown, text, json, or html)
/// with configurable inclusion of metadata, timestamps, provider info, and system prompt.
#[tauri::command]
pub async fn export_session(
    pool: State<'_, SqlitePool>,
    session_id: String,
    options: ExportOptions,
) -> Result<String, String> {
    let session_row = sqlx::query("SELECT * FROM sessions WHERE id = ?")
        .bind(&session_id)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| format!("DB error: {}", e))?
        .ok_or_else(|| "Session not found".to_string())?;

    let session = ExportSession {
        id: session_row.get("id"),
        title: session_row.get("title"),
        provider: session_row.try_get("provider").ok(),
        model: session_row.try_get("model").ok(),
        system_prompt: session_row.try_get("system_prompt").ok(),
        created_at: session_row.try_get("created_at").ok(),
        updated_at: session_row.try_get("updated_at").ok(),
        token_count: session_row.try_get::<i64, _>("token_count").ok(),
    };

    let msg_rows = sqlx::query(
        "SELECT * FROM messages WHERE session_id = ? AND is_deleted = 0 ORDER BY created_at ASC",
    )
    .bind(&session_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| format!("DB error: {}", e))?;

    let messages: Vec<ExportMessage> = msg_rows
        .iter()
        .map(|row| ExportMessage {
            id: row.get("id"),
            role: row.get("role"),
            content: row.get("content"),
            created_at: row.try_get("created_at").ok(),
            tokens_used: row.try_get("tokens_used").ok(),
            is_deleted: row.try_get("is_deleted").ok(),
            edited_at: row.try_get("edited_at").ok(),
        })
        .collect();

    match options.format.as_str() {
        "markdown" => export_markdown(&session, &messages, &options),
        "text" => export_text(&session, &messages, &options),
        "json" => Ok(export_json(&session, &messages)),
        "html" => export_html(&session, &messages, &options),
        _ => Err(format!("Unsupported format: {}", options.format)),
    }
}

fn export_markdown(
    session: &ExportSession,
    messages: &[ExportMessage],
    options: &ExportOptions,
) -> Result<String, String> {
    let mut md = String::new();

    md.push_str(&format!("# {}\n\n", session.title));

    if options.include_metadata {
        md.push_str("## Session Info\n\n");
        md.push_str(&format!("- **Created**: {}\n", session.created_at.as_deref().unwrap_or("N/A")));
        md.push_str(&format!("- **Updated**: {}\n", session.updated_at.as_deref().unwrap_or("N/A")));
        if let Some(count) = session.token_count {
            md.push_str(&format!("- **Total tokens**: {}\n", count));
        }
        if options.include_provider_info {
            if let Some(ref provider) = session.provider {
                md.push_str(&format!("- **Provider**: {}\n", provider));
            }
            if let Some(ref model) = session.model {
                md.push_str(&format!("- **Model**: {}\n", model));
            }
        }
        if options.include_system_prompt {
            if let Some(ref prompt) = session.system_prompt {
                if !prompt.is_empty() {
                    md.push_str(&format!("\n### System Prompt\n\n```\n{}\n```\n", prompt));
                }
            }
        }
        md.push('\n');
    }

    md.push_str("---\n\n");

    for msg in messages {
        if msg.is_deleted == Some(true) {
            md.push_str("> *[message deleted]*\n\n");
            continue;
        }

        let role_label = match msg.role.as_str() {
            "user" => "**You**",
            "assistant" => "**Assistant**",
            "system" => "**System**",
            _ => &msg.role,
        };

        md.push_str(&format!("{}", role_label));

        if options.include_timestamps {
            if let Some(ref ts) = msg.created_at {
                md.push_str(&format!(" — {}", ts.to_string()));
            }
        }

        if msg.edited_at.is_some() {
            md.push_str(" *(edited)*");
        }

        md.push_str("\n\n");
        md.push_str(&msg.content);
        md.push_str("\n\n");
    }

    Ok(md)
}

fn export_text(
    session: &ExportSession,
    messages: &[ExportMessage],
    options: &ExportOptions,
) -> Result<String, String> {
    let mut txt = String::new();

    txt.push_str(&format!("{}\n", session.title));
    txt.push_str(&format!("{}\n\n", "=".repeat(session.title.len())));

    if options.include_metadata {
        txt.push_str(&format!("Created: {}\n", session.created_at.as_deref().unwrap_or("N/A")));
        txt.push_str(&format!("Updated: {}\n", session.updated_at.as_deref().unwrap_or("N/A")));
        if options.include_provider_info {
            if let Some(ref provider) = session.provider {
                txt.push_str(&format!("Provider: {}\n", provider));
            }
            if let Some(ref model) = session.model {
                txt.push_str(&format!("Model: {}\n", model));
            }
        }
        txt.push('\n');
    }

    txt.push_str("---\n\n");

    for msg in messages {
        if msg.is_deleted == Some(true) {
            txt.push_str("[message deleted]\n\n");
            continue;
        }

        let role_label = match msg.role.as_str() {
            "user" => "You",
            "assistant" => "Assistant",
            "system" => "System",
            _ => &msg.role,
        };

        txt.push_str(&format!("{}", role_label));

        if options.include_timestamps {
            if let Some(ref ts) = msg.created_at {
                txt.push_str(&format!(" — {}", ts.to_string()));
            }
        }

        txt.push_str("\n\n");
        txt.push_str(&msg.content);
        txt.push_str("\n\n");
    }

    Ok(txt)
}

fn export_json(session: &ExportSession, messages: &[ExportMessage]) -> String {
    serde_json::to_string_pretty(&serde_json::json!({
        "session": {
            "id": session.id,
            "title": session.title,
            "provider": session.provider,
            "model": session.model,
            "system_prompt": session.system_prompt,
            "created_at": session.created_at,
            "updated_at": session.updated_at,
            "token_count": session.token_count,
        },
        "messages": messages.iter().map(|m| {
            serde_json::json!({
                "id": m.id,
                "role": m.role,
                "content": m.content,
                "created_at": m.created_at,
                "tokens_used": m.tokens_used,
                "is_deleted": m.is_deleted,
                "edited_at": m.edited_at,
            })
        }).collect::<Vec<_>>(),
    }))
    .unwrap_or_else(|_| "{}".to_string())
}

fn export_html(
    session: &ExportSession,
    messages: &[ExportMessage],
    options: &ExportOptions,
) -> Result<String, String> {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<style>
  body { font-family: system-ui, -apple-system, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; color: #1a1a2e; background: #fafafa; line-height: 1.6; }
  h1 { border-bottom: 1px solid #e0e0e0; padding-bottom: 10px; }
  .meta { font-size: 0.85em; color: #666; margin-bottom: 20px; }
  .meta dt { font-weight: 600; }
  .meta dd { margin-left: 0; margin-bottom: 4px; }
  .message { margin: 16px 0; padding: 12px 16px; border-radius: 8px; }
  .message.user { background: #e8eaf6; }
  .message.assistant { background: #f5f5f5; }
  .message .role { font-size: 0.75em; font-weight: 600; text-transform: uppercase; color: #888; margin-bottom: 4px; }
  .message .timestamp { font-size: 0.7em; color: #aaa; }
  .message .content { margin-top: 6px; white-space: pre-wrap; }
  .deleted { color: #999; font-style: italic; }
  hr { margin: 24px 0; border: none; border-top: 1px solid #e0e0e0; }
</style>
</head>
<body>
"#,
    );

    html.push_str(&format!("<h1>{}</h1>\n", escape_html(&session.title)));

    if options.include_metadata {
        html.push_str("<dl class=\"meta\">\n");
        if let Some(ref ts) = session.created_at {
            html.push_str(&format!("<dt>Created</dt><dd>{}</dd>\n", escape_html(ts)));
        }
        if let Some(ref ts) = session.updated_at {
            html.push_str(&format!("<dt>Updated</dt><dd>{}</dd>\n", escape_html(ts)));
        }
        if options.include_provider_info {
            if let Some(ref p) = session.provider {
                html.push_str(&format!("<dt>Provider</dt><dd>{}</dd>\n", escape_html(p)));
            }
            if let Some(ref m) = session.model {
                html.push_str(&format!("<dt>Model</dt><dd>{}</dd>\n", escape_html(m)));
            }
        }
        html.push_str("</dl>\n");
    }

    html.push_str("<hr>\n");

    for msg in messages {
        if msg.is_deleted == Some(true) {
            html.push_str("<div class=\"message deleted\">[message deleted]</div>\n");
            continue;
        }
        html.push_str(&format!("<div class=\"message {}\">\n", msg.role));
        html.push_str(&format!(
            "<div class=\"role\">{}</div>\n",
            escape_html(&msg.role)
        ));

        if options.include_timestamps {
            if let Some(ref ts) = msg.created_at {
                html.push_str(&format!(
                    "<div class=\"timestamp\">{}</div>\n",
                    escape_html(&ts.to_string())
                ));
            }
        }

        html.push_str(&format!(
            "<div class=\"content\">{}</div>\n",
            escape_html(&msg.content)
        ));
        html.push_str("</div>\n");
    }

    html.push_str("</body>\n</html>");

    Ok(html)
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_session() -> ExportSession {
        ExportSession {
            id: "sess-1".into(),
            title: "Test Chat".into(),
            provider: Some("DeepSeek".into()),
            model: Some("deepseek-v3".into()),
            system_prompt: Some("You are a helpful assistant.".into()),
            created_at: Some("2025-01-15T10:00:00Z".into()),
            updated_at: Some("2025-01-15T10:30:00Z".into()),
            token_count: Some(1500),
        }
    }

    fn sample_messages() -> Vec<ExportMessage> {
        vec![
            ExportMessage {
                id: "msg-1".into(),
                role: "user".into(),
                content: "Hello, how are you?".into(),
                created_at: Some("2025-01-15T10:00:00Z".into()),
                tokens_used: Some(10),
                is_deleted: Some(false),
                edited_at: None,
            },
            ExportMessage {
                id: "msg-2".into(),
                role: "assistant".into(),
                content: "I'm doing great, thanks!".into(),
                created_at: Some("2025-01-15T10:00:05Z".into()),
                tokens_used: Some(20),
                is_deleted: Some(false),
                edited_at: None,
            },
            ExportMessage {
                id: "msg-3".into(),
                role: "user".into(),
                content: "Can you help with Rust?".into(),
                created_at: Some("2025-01-15T10:01:00Z".into()),
                tokens_used: Some(15),
                is_deleted: Some(false),
                edited_at: None,
            },
        ]
    }

    fn all_options() -> ExportOptions {
        ExportOptions {
            format: "markdown".into(),
            include_metadata: true,
            include_timestamps: true,
            include_provider_info: true,
            include_system_prompt: true,
        }
    }

    fn no_options() -> ExportOptions {
        ExportOptions {
            format: "markdown".into(),
            include_metadata: false,
            include_timestamps: false,
            include_provider_info: false,
            include_system_prompt: false,
        }
    }

    // MARKDOWN TESTS

    #[test]
    fn markdown_all_options() {
        let result = export_markdown(&sample_session(), &sample_messages(), &all_options()).unwrap();
        assert!(result.contains("# Test Chat"));
        assert!(result.contains("**You**"));
        assert!(result.contains("**Assistant**"));
        assert!(result.contains("## Session Info"));
        assert!(result.contains("2025-01-15T10:00:00Z"));
        assert!(result.contains("DeepSeek"));
        assert!(result.contains("deepseek-v3"));
        assert!(result.contains("You are a helpful assistant"));
    }

    #[test]
    fn markdown_no_options() {
        let result = export_markdown(&sample_session(), &sample_messages(), &no_options()).unwrap();
        assert!(result.contains("# Test Chat"));
        assert!(!result.contains("## Session Info"));
        assert!(!result.contains("2025-01-15T10:00:00Z"));
        assert!(!result.contains("**Provider**"));
    }

    #[test]
    fn markdown_with_deleted_messages() {
        let mut msgs = sample_messages();
        msgs[0].is_deleted = Some(true);
        let result = export_markdown(&sample_session(), &msgs, &all_options()).unwrap();
        assert!(result.contains("[message deleted]"));
    }

    // TEXT TESTS

    #[test]
    fn text_all_options() {
        let result = export_text(&sample_session(), &sample_messages(), &all_options()).unwrap();
        assert!(result.contains("Test Chat"));
        assert!(result.contains("You"));
        assert!(result.contains("Assistant"));
        assert!(result.contains("Created:"));
        assert!(result.contains("DeepSeek"));
    }

    #[test]
    fn text_no_options() {
        let result = export_text(&sample_session(), &sample_messages(), &no_options()).unwrap();
        assert!(result.contains("Test Chat"));
        assert!(!result.contains("Created:"));
        assert!(!result.contains("Provider:"));
    }

    // JSON TESTS

    #[test]
    fn json_structure() {
        let result = export_json(&sample_session(), &sample_messages());
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(parsed.get("session").is_some());
        let messages = parsed.get("messages").unwrap().as_array().unwrap();
        assert_eq!(messages.len(), 3);
        let first = &messages[0];
        assert_eq!(first["role"], "user");
        assert!(first.get("content").is_some());
    }

    #[test]
    fn json_includes_session_metadata() {
        let result = export_json(&sample_session(), &sample_messages());
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        let session = &parsed["session"];
        assert_eq!(session["id"], "sess-1");
        assert_eq!(session["provider"], "DeepSeek");
        assert_eq!(session["model"], "deepseek-v3");
        assert_eq!(session["system_prompt"], "You are a helpful assistant.");
        assert_eq!(session["token_count"], 1500);
    }

    #[test]
    fn json_handles_none_fields() {
        let session = ExportSession {
            provider: None,
            model: None,
            system_prompt: None,
            ..sample_session()
        };
        let result = export_json(&session, &sample_messages());
        let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(parsed["session"]["provider"].is_null());
        assert!(parsed["session"]["model"].is_null());
        assert!(parsed["session"]["system_prompt"].is_null());
    }

    // HTML TESTS

    #[test]
    fn html_all_options() {
        let result = export_html(&sample_session(), &sample_messages(), &all_options()).unwrap();
        assert!(result.contains("<!DOCTYPE html>"));
        assert!(result.contains("<html lang=\"en\">"));
        assert!(result.contains("<meta charset=\"UTF-8\">"));
        assert!(result.contains("class=\"message user\""));
        assert!(result.contains("class=\"message assistant\""));
        assert!(result.contains("class=\"role\""));
        assert!(result.contains("class=\"timestamp\""));
        assert!(result.contains("DeepSeek"));
    }

    #[test]
    fn html_no_timestamps() {
        let mut opts = all_options();
        opts.include_timestamps = false;
        let result = export_html(&sample_session(), &sample_messages(), &opts).unwrap();
        assert!(!result.contains("class=\"timestamp\""));
    }

    #[test]
    fn html_no_provider_info() {
        let mut opts = all_options();
        opts.include_provider_info = false;
        let result = export_html(&sample_session(), &sample_messages(), &opts).unwrap();
        assert!(!result.contains("DeepSeek"));
        assert!(!result.contains("deepseek-v3"));
    }

    // ESCAPE HTML TESTS

    #[test]
    fn escape_html_ampersand() {
        assert_eq!(escape_html("a & b"), "a &amp; b");
    }

    #[test]
    fn escape_html_angle_brackets() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
    }

    #[test]
    fn escape_html_quotes() {
        assert_eq!(escape_html("\"hello\""), "&quot;hello&quot;");
        assert_eq!(escape_html("it's"), "it&#39;s");
    }

    #[test]
    fn escape_html_no_special_chars() {
        let input = "Hello, world!";
        assert_eq!(escape_html(input), input);
    }
}
