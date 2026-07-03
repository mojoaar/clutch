use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use std::time::Duration;

const CACHE_TTL_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub category: Option<String>,
    pub context_length: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ModelCacheEntry {
    pub provider: String,
    pub models: String,
    pub last_updated: String,
    pub etag: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    DeepSeek,
    OpenCodeGo,
    OpenCodeZen,
}

impl Provider {
    /// Returns the provider identifier string used for database storage and API lookups.
    pub fn as_str(&self) -> &str {
        match self {
            Provider::DeepSeek => "deepseek",
            Provider::OpenCodeGo => "opencode_go",
            Provider::OpenCodeZen => "opencode_zen",
        }
    }

    /// Returns the base API URL for fetching the model list from this provider.
    pub fn api_url(&self) -> String {
        if let Ok(base) = std::env::var("CLUTCH_API_BASE_OVERRIDE") {
            format!("{}/{}", base, self.as_str())
        } else {
            match self {
                Provider::DeepSeek => "https://api.deepseek.com/v1/models".to_string(),
                Provider::OpenCodeGo => "https://opencode.ai/zen/go/v1/models".to_string(),
                Provider::OpenCodeZen => "https://opencode.ai/zen/v1/models".to_string(),
            }
        }
    }
}

/// Returns the hardcoded default model list for a provider.
/// Used as a fallback when the API is unreachable and the cache is empty.
pub fn default_models(provider: &Provider) -> Vec<ModelInfo> {
    match provider {
        Provider::DeepSeek => vec![
            ModelInfo {
                id: "deepseek-v4-pro".into(),
                name: "DeepSeek V4 Pro".into(),
                provider: "deepseek".into(),
                category: Some("DeepSeek".into()),
                context_length: Some(1048576),
            },
            ModelInfo {
                id: "deepseek-v4-flash".into(),
                name: "DeepSeek V4 Flash".into(),
                provider: "deepseek".into(),
                category: Some("DeepSeek".into()),
                context_length: Some(1048576),
            },
        ],
        Provider::OpenCodeGo => vec![
            ModelInfo { id: "minimax-m3".into(), name: "MiniMax M3".into(), provider: "opencode_go".into(), category: Some("MiniMax".into()), context_length: None },
            ModelInfo { id: "minimax-m2.7".into(), name: "MiniMax M2.7".into(), provider: "opencode_go".into(), category: Some("MiniMax".into()), context_length: None },
            ModelInfo { id: "minimax-m2.5".into(), name: "MiniMax M2.5".into(), provider: "opencode_go".into(), category: Some("MiniMax".into()), context_length: None },
            ModelInfo { id: "kimi-k2.7-code".into(), name: "Kimi K2.7 Code".into(), provider: "opencode_go".into(), category: Some("Kimi".into()), context_length: None },
            ModelInfo { id: "kimi-k2.6".into(), name: "Kimi K2.6".into(), provider: "opencode_go".into(), category: Some("Kimi".into()), context_length: None },
            ModelInfo { id: "kimi-k2.5".into(), name: "Kimi K2.5".into(), provider: "opencode_go".into(), category: Some("Kimi".into()), context_length: None },
            ModelInfo { id: "glm-5.2".into(), name: "GLM 5.2".into(), provider: "opencode_go".into(), category: Some("GLM".into()), context_length: None },
            ModelInfo { id: "glm-5.1".into(), name: "GLM 5.1".into(), provider: "opencode_go".into(), category: Some("GLM".into()), context_length: None },
            ModelInfo { id: "glm-5".into(), name: "GLM 5".into(), provider: "opencode_go".into(), category: Some("GLM".into()), context_length: None },
            ModelInfo { id: "deepseek-v4-pro".into(), name: "DeepSeek V4 Pro".into(), provider: "opencode_go".into(), category: Some("DeepSeek".into()), context_length: None },
            ModelInfo { id: "deepseek-v4-flash".into(), name: "DeepSeek V4 Flash".into(), provider: "opencode_go".into(), category: Some("DeepSeek".into()), context_length: None },
            ModelInfo { id: "qwen3.7-max".into(), name: "Qwen 3.7 Max".into(), provider: "opencode_go".into(), category: Some("Qwen".into()), context_length: None },
            ModelInfo { id: "qwen3.7-plus".into(), name: "Qwen 3.7 Plus".into(), provider: "opencode_go".into(), category: Some("Qwen".into()), context_length: None },
            ModelInfo { id: "qwen3.6-plus".into(), name: "Qwen 3.6 Plus".into(), provider: "opencode_go".into(), category: Some("Qwen".into()), context_length: None },
            ModelInfo { id: "qwen3.5-plus".into(), name: "Qwen 3.5 Plus".into(), provider: "opencode_go".into(), category: Some("Qwen".into()), context_length: None },
            ModelInfo { id: "mimo-v2-pro".into(), name: "Mimo V2 Pro".into(), provider: "opencode_go".into(), category: Some("Mimo".into()), context_length: None },
            ModelInfo { id: "mimo-v2-omni".into(), name: "Mimo V2 Omni".into(), provider: "opencode_go".into(), category: Some("Mimo".into()), context_length: None },
            ModelInfo { id: "mimo-v2.5-pro".into(), name: "Mimo V2.5 Pro".into(), provider: "opencode_go".into(), category: Some("Mimo".into()), context_length: None },
            ModelInfo { id: "mimo-v2.5".into(), name: "Mimo V2.5".into(), provider: "opencode_go".into(), category: Some("Mimo".into()), context_length: None },
            ModelInfo { id: "hy3-preview".into(), name: "HY3 Preview".into(), provider: "opencode_go".into(), category: Some("Other".into()), context_length: None },
        ],
        Provider::OpenCodeZen => vec![
            ModelInfo { id: "claude-fable-5".into(), name: "Claude Fable 5".into(), provider: "opencode_zen".into(), category: Some("Claude".into()), context_length: None },
            ModelInfo { id: "claude-opus-4-8".into(), name: "Claude Opus 4.8".into(), provider: "opencode_zen".into(), category: Some("Claude".into()), context_length: None },
            ModelInfo { id: "claude-opus-4-7".into(), name: "Claude Opus 4.7".into(), provider: "opencode_zen".into(), category: Some("Claude".into()), context_length: None },
            ModelInfo { id: "claude-opus-4-6".into(), name: "Claude Opus 4.6".into(), provider: "opencode_zen".into(), category: Some("Claude".into()), context_length: None },
            ModelInfo { id: "claude-opus-4-5".into(), name: "Claude Opus 4.5".into(), provider: "opencode_zen".into(), category: Some("Claude".into()), context_length: None },
            ModelInfo { id: "claude-opus-4-1".into(), name: "Claude Opus 4.1".into(), provider: "opencode_zen".into(), category: Some("Claude".into()), context_length: None },
            ModelInfo { id: "claude-sonnet-4-6".into(), name: "Claude Sonnet 4.6".into(), provider: "opencode_zen".into(), category: Some("Claude".into()), context_length: None },
            ModelInfo { id: "claude-sonnet-4-5".into(), name: "Claude Sonnet 4.5".into(), provider: "opencode_zen".into(), category: Some("Claude".into()), context_length: None },
            ModelInfo { id: "claude-sonnet-4".into(), name: "Claude Sonnet 4".into(), provider: "opencode_zen".into(), category: Some("Claude".into()), context_length: None },
            ModelInfo { id: "claude-haiku-4-5".into(), name: "Claude Haiku 4.5".into(), provider: "opencode_zen".into(), category: Some("Claude".into()), context_length: None },
            ModelInfo { id: "gemini-3.5-flash".into(), name: "Gemini 3.5 Flash".into(), provider: "opencode_zen".into(), category: Some("Gemini".into()), context_length: None },
            ModelInfo { id: "gemini-3.1-pro".into(), name: "Gemini 3.1 Pro".into(), provider: "opencode_zen".into(), category: Some("Gemini".into()), context_length: None },
            ModelInfo { id: "gemini-3-flash".into(), name: "Gemini 3 Flash".into(), provider: "opencode_zen".into(), category: Some("Gemini".into()), context_length: None },
            ModelInfo { id: "gpt-5.5".into(), name: "GPT 5.5".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.5-pro".into(), name: "GPT 5.5 Pro".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.4".into(), name: "GPT 5.4".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.4-pro".into(), name: "GPT 5.4 Pro".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.4-mini".into(), name: "GPT 5.4 Mini".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.4-nano".into(), name: "GPT 5.4 Nano".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.3-codex-spark".into(), name: "GPT 5.3 Codex Spark".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.3-codex".into(), name: "GPT 5.3 Codex".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.2".into(), name: "GPT 5.2".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.2-codex".into(), name: "GPT 5.2 Codex".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.1".into(), name: "GPT 5.1".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.1-codex-max".into(), name: "GPT 5.1 Codex Max".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.1-codex".into(), name: "GPT 5.1 Codex".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5.1-codex-mini".into(), name: "GPT 5.1 Codex Mini".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5".into(), name: "GPT 5".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5-codex".into(), name: "GPT 5 Codex".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "gpt-5-nano".into(), name: "GPT 5 Nano".into(), provider: "opencode_zen".into(), category: Some("GPT".into()), context_length: None },
            ModelInfo { id: "grok-build-0.1".into(), name: "Grok Build 0.1".into(), provider: "opencode_zen".into(), category: Some("Grok".into()), context_length: None },
            ModelInfo { id: "deepseek-v4-pro".into(), name: "DeepSeek V4 Pro".into(), provider: "opencode_zen".into(), category: Some("DeepSeek".into()), context_length: None },
            ModelInfo { id: "deepseek-v4-flash".into(), name: "DeepSeek V4 Flash".into(), provider: "opencode_zen".into(), category: Some("DeepSeek".into()), context_length: None },
            ModelInfo { id: "glm-5.2".into(), name: "GLM 5.2".into(), provider: "opencode_zen".into(), category: Some("GLM".into()), context_length: None },
            ModelInfo { id: "glm-5.1".into(), name: "GLM 5.1".into(), provider: "opencode_zen".into(), category: Some("GLM".into()), context_length: None },
            ModelInfo { id: "glm-5".into(), name: "GLM 5".into(), provider: "opencode_zen".into(), category: Some("GLM".into()), context_length: None },
            ModelInfo { id: "minimax-m2.7".into(), name: "MiniMax M2.7".into(), provider: "opencode_zen".into(), category: Some("MiniMax".into()), context_length: None },
            ModelInfo { id: "minimax-m2.5".into(), name: "MiniMax M2.5".into(), provider: "opencode_zen".into(), category: Some("MiniMax".into()), context_length: None },
            ModelInfo { id: "kimi-k2.6".into(), name: "Kimi K2.6".into(), provider: "opencode_zen".into(), category: Some("Kimi".into()), context_length: None },
            ModelInfo { id: "kimi-k2.5".into(), name: "Kimi K2.5".into(), provider: "opencode_zen".into(), category: Some("Kimi".into()), context_length: None },
            ModelInfo { id: "qwen3.6-plus".into(), name: "Qwen 3.6 Plus".into(), provider: "opencode_zen".into(), category: Some("Qwen".into()), context_length: None },
            ModelInfo { id: "qwen3.5-plus".into(), name: "Qwen 3.5 Plus".into(), provider: "opencode_zen".into(), category: Some("Qwen".into()), context_length: None },
            ModelInfo { id: "big-pickle".into(), name: "Big Pickle".into(), provider: "opencode_zen".into(), category: Some("Other".into()), context_length: None },
            ModelInfo { id: "deepseek-v4-flash-free".into(), name: "DeepSeek V4 Flash Free".into(), provider: "opencode_zen".into(), category: Some("Free".into()), context_length: None },
            ModelInfo { id: "mimo-v2.5-free".into(), name: "Mimo V2.5 Free".into(), provider: "opencode_zen".into(), category: Some("Free".into()), context_length: None },
            ModelInfo { id: "qwen3.6-plus-free".into(), name: "Qwen 3.6 Plus Free".into(), provider: "opencode_zen".into(), category: Some("Free".into()), context_length: None },
            ModelInfo { id: "minimax-m3-free".into(), name: "MiniMax M3 Free".into(), provider: "opencode_zen".into(), category: Some("Free".into()), context_length: None },
            ModelInfo { id: "nemotron-3-ultra-free".into(), name: "Nemotron 3 Ultra Free".into(), provider: "opencode_zen".into(), category: Some("Free".into()), context_length: None },
            ModelInfo { id: "north-mini-code-free".into(), name: "North Mini Code Free".into(), provider: "opencode_zen".into(), category: Some("Free".into()), context_length: None },
        ],
    }
}

/// Returns the model list for a provider, using the cache if fresh or fetching from the API otherwise.
/// DeepSeek always returns the hardcoded default list without hitting the API.
pub async fn get_or_refresh_models(
    pool: &SqlitePool,
    provider: &Provider,
    api_key: &str,
) -> Result<Vec<ModelInfo>, Box<dyn std::error::Error + Send + Sync>> {
    if matches!(provider, Provider::DeepSeek) {
        return Ok(default_models(provider));
    }

    let cached = sqlx::query_as::<_, ModelCacheEntry>(
        "SELECT provider, models, last_updated, etag, version FROM model_cache WHERE provider = ?",
    )
    .bind(provider.as_str())
    .fetch_optional(pool)
    .await?;

    if let Some(ref entry) = cached {
        if let Ok(updated) = entry.last_updated.parse::<chrono::DateTime<Utc>>() {
            let age = Utc::now().signed_duration_since(updated);
            if age < chrono::TimeDelta::hours(CACHE_TTL_HOURS) {
                if let Ok(models) = serde_json::from_str::<Vec<ModelInfo>>(&entry.models) {
                    if !models.is_empty() {
                        return Ok(models);
                    }
                }
            }
        }
    }

    let cached_etag = cached.as_ref().map(|e| e.etag.as_str()).unwrap_or("");
    let fetched = fetch_models_from_api(provider, api_key, cached_etag).await;

    match fetched {
        Ok((models, etag)) => {
            let json = serde_json::to_string(&models)?;
            sqlx::query(
                "INSERT INTO model_cache (provider, models, last_updated, etag, version)
                 VALUES (?, ?, datetime('now'), ?, '1')
                 ON CONFLICT(provider) DO UPDATE SET
                 models = excluded.models,
                 last_updated = excluded.last_updated,
                 etag = excluded.etag,
                 version = CAST(COALESCE(CAST(model_cache.version AS INTEGER), 0) + 1 AS TEXT)",
            )
            .bind(provider.as_str())
            .bind(&json)
            .bind(&etag)
            .execute(pool)
            .await?;

            Ok(models)
        }
        Err(_) => {
            if let Some(entry) = cached {
                if let Ok(models) = serde_json::from_str::<Vec<ModelInfo>>(&entry.models) {
                    if !models.is_empty() {
                        return Ok(models);
                    }
                }
            }
            Ok(default_models(provider))
        }
    }
}

async fn fetch_models_from_api(
    provider: &Provider,
    api_key: &str,
    _etag: &str,
) -> Result<(Vec<ModelInfo>, String), Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let resp = client
        .get(provider.api_url())
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;

    if resp.status().is_success() {
        let body = resp.text().await?;
        let models = parse_model_response(provider, &body);
        Ok((models, String::new()))
    } else {
        Err(format!("API returned status {}", resp.status()).into())
    }
}

fn categorize_model(id: &str) -> &str {
    if id.starts_with("minimax-") { "MiniMax" }
    else if id.starts_with("kimi-") { "Kimi" }
    else if id.starts_with("glm-") { "GLM" }
    else if id.starts_with("deepseek-") { "DeepSeek" }
    else if id.starts_with("qwen") { "Qwen" }
    else if id.starts_with("mimo-") { "Mimo" }
    else if id.starts_with("claude-") { "Claude" }
    else if id.starts_with("gemini-") { "Gemini" }
    else if id.starts_with("gpt-") { "GPT" }
    else if id.starts_with("grok-") { "Grok" }
    else if id.starts_with("hy") { "Other" }
    else if id.starts_with("big-pickle") { "Other" }
    else if id.starts_with("nemotron-") { "Free" }
    else if id.starts_with("north-") { "Free" }
    else if id.contains("free") { "Free" }
    else { id.split('-').next().unwrap_or("Other") }
}

fn title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(chars).collect(),
    }
}

fn format_model_name(id: &str) -> String {
    let segments: Vec<&str> = id.split('-').collect();
    let mut result: Vec<String> = Vec::new();
    let mut i = 0;
    while i < segments.len() {
        let seg = segments[i];
        let is_num = seg.chars().all(|c| c.is_ascii_digit() || c == '.');
        if is_num && i > 0 {
            let prev_seg = segments[i - 1];
            let prev_is_num = prev_seg.chars().all(|c| c.is_ascii_digit() || c == '.');
            if prev_is_num {
                let prev = result.pop().unwrap_or_default();
                result.push(format!("{}.{}", prev, seg));
                i += 1;
                continue;
            }
        }
        result.push(title_case(seg));
        i += 1;
    }
    result.join(" ")
}

fn parse_model_response(provider: &Provider, body: &str) -> Vec<ModelInfo> {
    if let Ok(json) = serde_json::from_str::<serde_json::Value>(body) {
        if let Some(data) = json.get("data").or_else(|| json.get("models")) {
            if let Some(models) = data.as_array() {
                return models
                    .iter()
                    .filter_map(|m| {
                        let id = m.get("id")?.as_str()?;
                        let name = format_model_name(id);
                        Some(ModelInfo {
                            id: id.to_string(),
                            name,
                            provider: provider.as_str().to_string(),
                            category: Some(categorize_model(id).to_string()),
                            context_length: Some(crate::context::get_model_limit(id) as i64),
                        })
                    })
                    .collect();
            }
        }
    }

    default_models(provider)
}

/// Tauri command to get available models for a provider, refreshing from the API if the cache is stale.
#[tauri::command]
pub async fn get_models(
    pool: tauri::State<'_, SqlitePool>,
    provider: String,
    api_key: Option<String>,
) -> Result<Vec<ModelInfo>, String> {
    let provider = match provider.as_str() {
        "deepseek" => Provider::DeepSeek,
        "opencode_go" => Provider::OpenCodeGo,
        "opencode_zen" => Provider::OpenCodeZen,
        _ => return Err(format!("Unknown provider: {}", provider)),
    };

    let key = api_key.as_deref().unwrap_or("");
    get_or_refresh_models(&pool, &provider, key)
        .await
        .map_err(|e| e.to_string())
}

/// Tauri command to force a refresh of the model list from the provider API, bypassing cache TTL.
#[tauri::command]
pub async fn refresh_models(
    pool: tauri::State<'_, SqlitePool>,
    provider: String,
    api_key: Option<String>,
) -> Result<Vec<ModelInfo>, String> {
    let provider = match provider.as_str() {
        "deepseek" => Provider::DeepSeek,
        "opencode_go" => Provider::OpenCodeGo,
        "opencode_zen" => Provider::OpenCodeZen,
        _ => return Err(format!("Unknown provider: {}", provider)),
    };
    if matches!(provider, Provider::DeepSeek) {
        return Ok(default_models(&provider));
    }
    let key = api_key.as_deref().unwrap_or("");
    let fetched = fetch_models_from_api(&provider, key, "").await;
    match fetched {
        Ok((models, etag)) => {
            let json = serde_json::to_string(&models).unwrap_or_default();
            let _ = sqlx::query(
                "INSERT INTO model_cache (provider, models, last_updated, etag, version) VALUES (?, ?, datetime('now'), ?, '1') ON CONFLICT(provider) DO UPDATE SET models = excluded.models, last_updated = excluded.last_updated, etag = excluded.etag, version = CAST(COALESCE(CAST(model_cache.version AS INTEGER), 0) + 1 AS TEXT)",
            ).bind(provider.as_str()).bind(&json).bind(&etag).execute(pool.inner()).await;
            Ok(models)
        }
        Err(_) => Ok(default_models(&provider)),
    }
}

/// Tauri command to retrieve models from the local cache without performing any API requests.
/// Falls back to refetching from the API if the cache is empty for a given provider.
#[tauri::command]
pub async fn get_cached_models(
    pool: tauri::State<'_, SqlitePool>,
    provider: String,
) -> Result<Vec<ModelInfo>, String> {
    let row = sqlx::query_as::<_, ModelCacheEntry>(
        "SELECT provider, models, last_updated, etag, version FROM model_cache WHERE provider = ?",
    )
    .bind(&provider)
    .fetch_optional(pool.inner())
    .await
    .map_err(|e| format!("DB error: {}", e))?;

    if let Some(entry) = row {
        if let Ok(models) = serde_json::from_str::<Vec<ModelInfo>>(&entry.models) {
            if !models.is_empty() {
                return Ok(models);
            }
        }
    }

    let p = match provider.as_str() {
        "deepseek" => Provider::DeepSeek,
        "opencode_go" => Provider::OpenCodeGo,
        "opencode_zen" => Provider::OpenCodeZen,
        _ => Provider::DeepSeek,
    };

    if matches!(p, Provider::DeepSeek) {
        return Ok(default_models(&p));
    }

    get_or_refresh_models(&pool, &p, "")
        .await
        .map_err(|e| e.to_string())
}
