mod common;

use common::create_test_pool;
use tauri::Manager;

#[tokio::test]
async fn create_session_returns_row() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(
        app.state(),
        "s1".into(),
        "Test Session".into(),
        "deepseek".into(),
        "deepseek-v4-pro".into(),
        None,
    ).await.unwrap();
    let row: (String,) = sqlx::query_as("SELECT title FROM sessions WHERE id = ?")
        .bind("s1").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "Test Session");
}

#[tokio::test]
async fn create_session_stores_system_prompt() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(
        app.state(), "s2".into(), "With SP".into(), "deepseek".into(), "deepseek-v4-pro".into(),
        Some("You are helpful".into()),
    ).await.unwrap();
    let row: (Option<String>,) = sqlx::query_as("SELECT system_prompt FROM sessions WHERE id = ?")
        .bind("s2").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0.as_deref(), Some("You are helpful"));
}

#[tokio::test]
async fn list_sessions_excludes_archived_by_default() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "a1".into(), "Active".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::create_session(app.state(), "a2".into(), "Archived".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    sqlx::query("UPDATE sessions SET is_archived = 1 WHERE id = 'a2'").execute(&pool).await.unwrap();
    let sessions = app_lib::sessions::list_sessions(app.state(), Some(false)).await.unwrap();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0].id, "a1");
}

#[tokio::test]
async fn list_sessions_include_archived() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "b1".into(), "A".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::create_session(app.state(), "b2".into(), "B".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    sqlx::query("UPDATE sessions SET is_archived = 1 WHERE id = 'b2'").execute(&pool).await.unwrap();
    let sessions = app_lib::sessions::list_sessions(app.state(), Some(true)).await.unwrap();
    assert_eq!(sessions.len(), 2);
}

#[tokio::test]
async fn update_session_title_persists() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "c1".into(), "Old".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::update_session_title(app.state(), "c1".into(), "New".into()).await.unwrap();
    let row: (String,) = sqlx::query_as("SELECT title FROM sessions WHERE id = ?")
        .bind("c1").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "New");
}

#[tokio::test]
async fn update_session_provider_changes_both() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "d1".into(), "T".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::update_session_provider(app.state(), "d1".into(), "opencode_go".into(), "kimi-latest".into()).await.unwrap();
    let row: (String, String) = sqlx::query_as("SELECT provider, model FROM sessions WHERE id = ?")
        .bind("d1").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "opencode_go");
    assert_eq!(row.1, "kimi-latest");
}

#[tokio::test]
async fn pin_and_unpin_session() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "e1".into(), "T".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::pin_session(app.state(), "e1".into()).await.unwrap();
    let row: (i64,) = sqlx::query_as("SELECT is_pinned FROM sessions WHERE id = ?")
        .bind("e1").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, 1);
    app_lib::sessions::unpin_session(app.state(), "e1".into()).await.unwrap();
    let row: (i64,) = sqlx::query_as("SELECT is_pinned FROM sessions WHERE id = ?")
        .bind("e1").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, 0);
}

#[tokio::test]
async fn archive_and_unarchive_session() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "f1".into(), "T".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::archive_session(app.state(), "f1".into()).await.unwrap();
    let row: (i64,) = sqlx::query_as("SELECT is_archived FROM sessions WHERE id = ?")
        .bind("f1").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, 1);
    app_lib::sessions::unarchive_session(app.state(), "f1".into()).await.unwrap();
    let row: (i64,) = sqlx::query_as("SELECT is_archived FROM sessions WHERE id = ?")
        .bind("f1").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, 0);
}

#[tokio::test]
async fn delete_session_cascades_messages() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "g1".into(), "T".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::create_message(app.state(), "m1".into(), "g1".into(), "user".into(), "Hello".into(), Some(0)).await.unwrap();
    app_lib::sessions::delete_session(app.state(), "g1".into()).await.unwrap();
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM messages WHERE session_id = ?")
        .bind("g1").fetch_one(&pool).await.unwrap();
    assert_eq!(count.0, 0);
}

#[tokio::test]
async fn list_messages_in_order() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "h1".into(), "T".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::create_message(app.state(), "m1".into(), "h1".into(), "user".into(), "First".into(), Some(0)).await.unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10));
    app_lib::sessions::create_message(app.state(), "m2".into(), "h1".into(), "assistant".into(), "Second".into(), Some(0)).await.unwrap();
    let msgs = app_lib::sessions::list_messages(app.state(), "h1".into()).await.unwrap();
    assert_eq!(msgs.len(), 2);
    assert_eq!(msgs[0].role, "user");
    assert_eq!(msgs[1].role, "assistant");
}

#[tokio::test]
async fn create_message_stores_all_fields() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "i1".into(), "T".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::create_message(app.state(), "mm".into(), "i1".into(), "user".into(), "Hi".into(), Some(42)).await.unwrap();
    let row: (String, String, i64) = sqlx::query_as("SELECT role, content, tokens_used FROM messages WHERE id = ?")
        .bind("mm").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "user");
    assert_eq!(row.1, "Hi");
    assert_eq!(row.2, 42);
}

#[tokio::test]
async fn update_message_changes_content() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "j1".into(), "T".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::create_message(app.state(), "mu".into(), "j1".into(), "user".into(), "Before".into(), Some(0)).await.unwrap();
    app_lib::sessions::update_message(app.state(), "mu".into(), "After".into()).await.unwrap();
    let row: (String,) = sqlx::query_as("SELECT content FROM messages WHERE id = ?")
        .bind("mu").fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "After");
}

#[tokio::test]
async fn delete_message_soft_deletes() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "k1".into(), "T".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::create_message(app.state(), "md".into(), "k1".into(), "user".into(), "Bye".into(), Some(0)).await.unwrap();
    app_lib::sessions::delete_message(app.state(), "md".into()).await.unwrap();
    let msgs = app_lib::sessions::list_messages(app.state(), "k1".into()).await.unwrap();
    assert_eq!(msgs.len(), 1);
    assert!(msgs[0].is_deleted);
}

#[tokio::test]
async fn pinned_sessions_sort_first() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "p1".into(), "A".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::create_session(app.state(), "p2".into(), "B".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    app_lib::sessions::pin_session(app.state(), "p2".into()).await.unwrap();
    let sessions = app_lib::sessions::list_sessions(app.state(), Some(false)).await.unwrap();
    assert_eq!(sessions[0].id, "p2");
    assert_eq!(sessions[1].id, "p1");
}

#[tokio::test]
async fn list_sessions_returns_empty() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    let sessions = app_lib::sessions::list_sessions(app.state(), Some(false)).await.unwrap();
    assert!(sessions.is_empty());
}

#[tokio::test]
async fn update_session_title_missing_id_no_panic() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    let result = app_lib::sessions::update_session_title(app.state(), "nonexistent".into(), "X".into()).await;
    result.unwrap();
}

#[tokio::test]
async fn no_duplicate_session_ids() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "dup".into(), "First".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    let result = app_lib::sessions::create_session(
        app.state(), "dup".into(), "Second".into(), "deepseek".into(), "deepseek-v4-pro".into(), None,
    ).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn list_messages_empty_session() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::sessions::create_session(app.state(), "empty".into(), "T".into(), "deepseek".into(), "deepseek-v4-pro".into(), None).await.unwrap();
    let msgs = app_lib::sessions::list_messages(app.state(), "empty".into()).await.unwrap();
    assert!(msgs.is_empty());
}

#[tokio::test]
async fn delete_session_nonexistent_no_panic() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    let result = app_lib::sessions::delete_session(app.state(), "ghost".into()).await;
    result.unwrap();
}

#[tokio::test]
async fn create_message_invalid_session_errors() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    let result = app_lib::sessions::create_message(
        app.state(), "m".into(), "no_session".into(), "user".into(), "Hi".into(), Some(0),
    ).await;
    assert!(result.is_err());
}
