mod common;

use common::create_test_pool;
use tauri::Manager;

#[test]
fn deepseek_endpoint() {
    assert_eq!(app_lib::api::provider_endpoint("deepseek"), "https://api.deepseek.com/v1/chat/completions");
}

#[test]
fn opencode_go_endpoint() {
    assert_eq!(app_lib::api::provider_endpoint("opencode_go"), "https://opencode.ai/zen/go/v1/chat/completions");
}

#[test]
fn opencode_zen_endpoint() {
    assert_eq!(app_lib::api::provider_endpoint("opencode_zen"), "https://opencode.ai/zen/v1/chat/completions");
}

#[test]
fn unknown_provider_falls_back_to_deepseek() {
    assert_eq!(app_lib::api::provider_endpoint("unknown"), "https://api.deepseek.com/v1/chat/completions");
}

#[test]
fn empty_string_falls_back_to_deepseek() {
    assert_eq!(app_lib::api::provider_endpoint(""), "https://api.deepseek.com/v1/chat/completions");
}

#[tokio::test]
async fn get_api_key_not_found_returns_err() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let res = app_lib::api::get_api_key_for_test(&app.handle(), &pool, "deepseek").await;
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("API key not found for provider: deepseek"));
}

#[tokio::test]
async fn get_api_key_plaintext_readable() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    // Directly insert a raw plaintext key
    sqlx::query("INSERT INTO settings (key, value) VALUES ('api_key_deepseek', 'sk-raw-plaintext')")
        .execute(&pool)
        .await
        .unwrap();

    let key = app_lib::api::get_api_key_for_test(&app.handle(), &pool, "deepseek").await.unwrap();
    assert_eq!(key, "sk-raw-plaintext");
}

#[tokio::test]
async fn get_api_key_encrypted_roundtrip() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    // Use set_setting (which encrypts API keys automatically)
    app_lib::settings::set_setting(
        app.handle().clone(),
        app.state(),
        "api_key_deepseek".to_string(),
        "sk-my-super-secret-key".to_string(),
    )
    .await
    .unwrap();

    // Verify it is encrypted in the DB
    let raw: (String,) = sqlx::query_as("SELECT value FROM settings WHERE key = 'api_key_deepseek'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!raw.0.contains("sk-my-super-secret-key"));

    // Decrypts on retrieval
    let key = app_lib::api::get_api_key_for_test(&app.handle(), &pool, "deepseek").await.unwrap();
    assert_eq!(key, "sk-my-super-secret-key");
}
