use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub provider: String,
    pub model: String,
    pub system_prompt: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub is_archived: bool,
    pub is_pinned: bool,
    pub token_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub created_at: String,
    pub tokens_used: Option<i64>,
    pub is_deleted: bool,
    pub edited_at: Option<String>,
}

/// Tauri command to list chat sessions, optionally including archived ones.
/// Results are ordered by pinned status first, then by last update time.
#[tauri::command]
pub async fn list_sessions(
    pool: State<'_, SqlitePool>,
    include_archived: Option<bool>,
) -> Result<Vec<Session>, String> {
    let include = include_archived.unwrap_or(false);
    let rows = if include {
        sqlx::query_as::<_, SessionRow>(
            "SELECT id, title, provider, model, system_prompt, created_at, updated_at, is_archived, is_pinned, token_count FROM sessions ORDER BY is_pinned DESC, updated_at DESC",
        )
        .fetch_all(pool.inner())
        .await
    } else {
        sqlx::query_as::<_, SessionRow>(
            "SELECT id, title, provider, model, system_prompt, created_at, updated_at, is_archived, is_pinned, token_count FROM sessions WHERE is_archived = 0 ORDER BY is_pinned DESC, updated_at DESC",
        )
        .fetch_all(pool.inner())
        .await
    }
    .map_err(|e| e.to_string())?;

    Ok(rows.into_iter().map(|r| r.into()).collect())
}

/// Tauri command to create a new chat session with the given provider, model, and optional system prompt.
#[tauri::command]
pub async fn create_session(
    pool: State<'_, SqlitePool>,
    id: String,
    title: String,
    provider: String,
    model: String,
    system_prompt: Option<String>,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO sessions (id, title, provider, model, system_prompt, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&title)
    .bind(&provider)
    .bind(&model)
    .bind(&system_prompt.unwrap_or_default())
    .bind(&now)
    .bind(&now)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

/// Tauri command to update the title of an existing chat session.
#[tauri::command]
pub async fn update_session_title(
    pool: State<'_, SqlitePool>,
    id: String,
    title: String,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query("UPDATE sessions SET title = ?, updated_at = ? WHERE id = ?")
        .bind(&title)
        .bind(&now)
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Tauri command to update the provider and model for an existing chat session.
#[tauri::command]
pub async fn update_session_provider(
    pool: State<'_, SqlitePool>,
    id: String,
    provider: String,
    model: String,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query("UPDATE sessions SET provider = ?, model = ?, updated_at = ? WHERE id = ?")
        .bind(&provider)
        .bind(&model)
        .bind(&now)
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Tauri command to pin a session so it appears at the top of the session list.
#[tauri::command]
pub async fn pin_session(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    sqlx::query("UPDATE sessions SET is_pinned = 1 WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Tauri command to unpin a previously pinned session.
#[tauri::command]
pub async fn unpin_session(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    sqlx::query("UPDATE sessions SET is_pinned = 0 WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Tauri command to archive a session, hiding it from the default session list.
#[tauri::command]
pub async fn archive_session(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    sqlx::query("UPDATE sessions SET is_archived = 1 WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Tauri command to unarchive a session, restoring it to the default session list.
#[tauri::command]
pub async fn unarchive_session(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    sqlx::query("UPDATE sessions SET is_archived = 0 WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Tauri command to permanently delete a session and all its associated messages.
#[tauri::command]
pub async fn delete_session(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    sqlx::query("DELETE FROM messages WHERE session_id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM sessions WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Tauri command to list all messages for a given session, ordered by creation time ascending.
#[tauri::command]
pub async fn list_messages(
    pool: State<'_, SqlitePool>,
    session_id: String,
) -> Result<Vec<Message>, String> {
    let rows = sqlx::query_as::<_, MessageRow>(
        "SELECT id, session_id, role, content, created_at, tokens_used, is_deleted, edited_at FROM messages WHERE session_id = ? ORDER BY created_at ASC",
    )
    .bind(&session_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows.into_iter().map(|r| r.into()).collect())
}

/// Tauri command to create a new message in a session, updating the session's token count.
#[tauri::command]
pub async fn create_message(
    pool: State<'_, SqlitePool>,
    id: String,
    session_id: String,
    role: String,
    content: String,
    tokens_used: Option<i64>,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO messages (id, session_id, role, content, created_at, tokens_used) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&session_id)
    .bind(&role)
    .bind(&content)
    .bind(&now)
    .bind(tokens_used)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    sqlx::query("UPDATE sessions SET updated_at = ?, token_count = token_count + COALESCE(?, 0) WHERE id = ?")
        .bind(&now)
        .bind(tokens_used.unwrap_or(0))
        .bind(&session_id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Tauri command to update the content of an existing message and set its edited timestamp.
#[tauri::command]
pub async fn update_message(
    pool: State<'_, SqlitePool>,
    id: String,
    content: String,
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query("UPDATE messages SET content = ?, edited_at = ? WHERE id = ?")
        .bind(&content)
        .bind(&now)
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Tauri command to soft-delete a message by setting its is_deleted flag.
#[tauri::command]
pub async fn delete_message(pool: State<'_, SqlitePool>, id: String) -> Result<(), String> {
    sqlx::query("UPDATE messages SET is_deleted = 1 WHERE id = ?")
        .bind(&id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

struct SessionRow {
    id: String,
    title: String,
    provider: String,
    model: String,
    system_prompt: Option<String>,
    created_at: String,
    updated_at: String,
    is_archived: bool,
    is_pinned: bool,
    token_count: i64,
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for SessionRow {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
        use sqlx::Row;
        Ok(SessionRow {
            id: row.try_get("id")?,
            title: row.try_get("title")?,
            provider: row.try_get("provider")?,
            model: row.try_get("model")?,
            system_prompt: row.try_get("system_prompt")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            is_archived: row.try_get::<i64, _>("is_archived").unwrap_or(0) != 0,
            is_pinned: row.try_get::<i64, _>("is_pinned").unwrap_or(0) != 0,
            token_count: row.try_get("token_count")?,
        })
    }
}

impl From<SessionRow> for Session {
    fn from(r: SessionRow) -> Self {
        Session {
            id: r.id,
            title: r.title,
            provider: r.provider,
            model: r.model,
            system_prompt: r.system_prompt,
            created_at: r.created_at,
            updated_at: r.updated_at,
            is_archived: r.is_archived,
            is_pinned: r.is_pinned,
            token_count: r.token_count,
        }
    }
}

struct MessageRow {
    id: String,
    session_id: String,
    role: String,
    content: String,
    created_at: String,
    tokens_used: Option<i64>,
    is_deleted: bool,
    edited_at: Option<String>,
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for MessageRow {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
        use sqlx::Row;
        Ok(MessageRow {
            id: row.try_get("id")?,
            session_id: row.try_get("session_id")?,
            role: row.try_get("role")?,
            content: row.try_get("content")?,
            created_at: row.try_get("created_at")?,
            tokens_used: row.try_get("tokens_used")?,
            is_deleted: row.try_get::<i64, _>("is_deleted").unwrap_or(0) != 0,
            edited_at: row.try_get("edited_at")?,
        })
    }
}

impl From<MessageRow> for Message {
    fn from(r: MessageRow) -> Self {
        Message {
            id: r.id,
            session_id: r.session_id,
            role: r.role,
            content: r.content,
            created_at: r.created_at,
            tokens_used: r.tokens_used,
            is_deleted: r.is_deleted,
            edited_at: r.edited_at,
        }
    }
}
