use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub display_name: String,
    pub avatar_type: String,
    pub avatar_data: String,
    pub avatar_color: String,
}

/// Tauri command to retrieve the user profile (display name, avatar type, avatar data, and color) from the database.
#[tauri::command]
pub async fn get_user_profile(
    pool: State<'_, SqlitePool>,
) -> Result<Option<UserProfile>, String> {
    let row = sqlx::query("SELECT display_name, avatar_type, avatar_data, avatar_color FROM user_profile WHERE id = 'default'")
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| format!("DB error: {}", e))?;

    match row {
        Some(r) => Ok(Some(UserProfile {
            display_name: r.get("display_name"),
            avatar_type: r.get("avatar_type"),
            avatar_data: r.get("avatar_data"),
            avatar_color: r.get("avatar_color"),
        })),
        None => Ok(None),
    }
}

/// Tauri command to upsert the user profile with display name, avatar type, avatar data, and avatar color.
#[tauri::command]
pub async fn update_user_profile(
    pool: State<'_, SqlitePool>,
    display_name: String,
    avatar_type: String,
    avatar_data: String,
    avatar_color: String,
) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO user_profile (id, display_name, avatar_type, avatar_data, avatar_color, updated_at)
         VALUES ('default', ?, ?, ?, ?, datetime('now'))
         ON CONFLICT(id) DO UPDATE SET
         display_name = excluded.display_name,
         avatar_type = excluded.avatar_type,
         avatar_data = excluded.avatar_data,
         avatar_color = excluded.avatar_color,
         updated_at = excluded.updated_at",
    )
    .bind(&display_name)
    .bind(&avatar_type)
    .bind(&avatar_data)
    .bind(&avatar_color)
    .execute(pool.inner())
    .await
    .map_err(|e| format!("DB error: {}", e))?;

    Ok(())
}
