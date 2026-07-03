mod common;

use common::create_test_pool;
use tauri::Manager;

#[tokio::test]
async fn get_all_settings_returns_values() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    sqlx::query("INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('test_a', '1', '2024-01-01')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('test_b', '2', '2024-01-01')")
        .execute(&pool).await.unwrap();
    let all = app_lib::settings::get_all_settings(app.state()).await.unwrap();
    assert_eq!(all.get("test_a").map(|s| s.as_str()), Some("1"));
    assert_eq!(all.get("test_b").map(|s| s.as_str()), Some("2"));
}

#[tokio::test]
async fn get_all_settings_skips_api_keys() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    sqlx::query("INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('api_key_deepseek', 'encrypted_sk_test', '2024-01-01')")
        .execute(&pool).await.unwrap();
    let all = app_lib::settings::get_all_settings(app.state()).await.unwrap();
    assert!(!all.contains_key("api_key_deepseek"));
}

#[tokio::test]
async fn get_all_settings_includes_defaults() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    let all = app_lib::settings::get_all_settings(app.state()).await.unwrap();
    assert!(!all.is_empty());
    assert!(all.contains_key("theme"));
}

#[tokio::test]
async fn get_all_settings_overwritten_value_persists() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    sqlx::query("INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('theme', 'dracula', '2024-01-01')")
        .execute(&pool).await.unwrap();
    let all = app_lib::settings::get_all_settings(app.state()).await.unwrap();
    assert_eq!(all.get("theme").map(|s| s.as_str()), Some("dracula"));
}
