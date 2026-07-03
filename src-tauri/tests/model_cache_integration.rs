mod common;

use common::create_test_pool;
use tauri::Manager;
use app_lib::model_cache::{Provider, default_models, get_models, get_cached_models, refresh_models};

static SERIAL_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn provider_as_str_mappings() {
    assert_eq!(Provider::DeepSeek.as_str(), "deepseek");
    assert_eq!(Provider::OpenCodeGo.as_str(), "opencode_go");
    assert_eq!(Provider::OpenCodeZen.as_str(), "opencode_zen");
}

#[test]
fn provider_api_url_mappings() {
    let _guard = SERIAL_LOCK.lock().unwrap();
    // Verified both default and env override mappings
    assert_eq!(Provider::DeepSeek.api_url(), "https://api.deepseek.com/v1/models");
    
    std::env::set_var("CLUTCH_API_BASE_OVERRIDE", "http://localhost:8080");
    assert_eq!(Provider::OpenCodeGo.api_url(), "http://localhost:8080/opencode_go");
    assert_eq!(Provider::OpenCodeZen.api_url(), "http://localhost:8080/opencode_zen");
    
    std::env::remove_var("CLUTCH_API_BASE_OVERRIDE");
}

#[test]
fn default_models_returns_correct_items() {
    let ds = default_models(&Provider::DeepSeek);
    assert_eq!(ds.len(), 2);
    assert_eq!(ds[0].id, "deepseek-v4-pro");
    assert_eq!(ds[1].id, "deepseek-v4-flash");

    let go = default_models(&Provider::OpenCodeGo);
    assert!(go.len() >= 10);
    assert_eq!(go[0].provider, "opencode_go");

    let zen = default_models(&Provider::OpenCodeZen);
    assert!(zen.len() >= 10);
    assert_eq!(zen[0].provider, "opencode_zen");
}

#[tokio::test]
async fn get_models_deepseek_shortcuts_network() {
    let _guard = SERIAL_LOCK.lock().unwrap();
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let list = get_models(app.state(), "deepseek".to_string(), None).await.unwrap();
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn refresh_models_deepseek_shortcuts_network() {
    let _guard = SERIAL_LOCK.lock().unwrap();
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let list = refresh_models(app.state(), "deepseek".to_string(), None).await.unwrap();
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn get_cached_models_deepseek_shortcuts_network() {
    let _guard = SERIAL_LOCK.lock().unwrap();
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let list = get_cached_models(app.state(), "deepseek".to_string()).await.unwrap();
    assert_eq!(list.len(), 2);
}

#[tokio::test]
async fn get_cached_models_returns_custom_cached_row() {
    let _guard = SERIAL_LOCK.lock().unwrap();
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let custom_models_json = r#"[
        {"id":"cached-model-1","name":"Cached Model 1","provider":"opencode_go","category":"Custom","context_length":8192}
    ]"#;

    sqlx::query(
        "INSERT INTO model_cache (provider, models, last_updated, etag, version)
         VALUES ('opencode_go', ?, datetime('now'), 'custom-etag', '1')"
    )
    .bind(custom_models_json)
    .execute(&pool)
    .await
    .unwrap();

    let list = get_cached_models(app.state(), "opencode_go".to_string()).await.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, "cached-model-1");
}

#[tokio::test]
async fn get_cached_models_empty_cache_falls_back_to_defaults() {
    let _guard = SERIAL_LOCK.lock().unwrap();
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let list = get_cached_models(app.state(), "opencode_go".to_string()).await.unwrap();
    assert!(list.len() > 5);
}

#[tokio::test]
async fn get_models_unknown_provider_returns_err() {
    let _guard = SERIAL_LOCK.lock().unwrap();
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let res = get_models(app.state(), "unknown_provider".to_string(), None).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn refresh_models_unknown_provider_returns_err() {
    let _guard = SERIAL_LOCK.lock().unwrap();
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let res = refresh_models(app.state(), "unknown_provider".to_string(), None).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn refresh_models_with_mock_http_server() {
    let _guard = SERIAL_LOCK.lock().unwrap();
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let mut server = mockito::Server::new_async().await;
    std::env::set_var("CLUTCH_API_BASE_OVERRIDE", server.url());

    let body_json = r#"{"data": [{"id": "qwen-mock-9b"}]}"#;
    let mock = server.mock("GET", "/opencode_go")
        .match_header("Authorization", "Bearer sk-test-key")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(body_json)
        .create_async()
        .await;

    let list = refresh_models(app.state(), "opencode_go".to_string(), Some("sk-test-key".to_string())).await.unwrap();
    mock.assert_async().await;
    
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, "qwen-mock-9b");
    assert_eq!(list[0].name, "Qwen Mock 9b");
    assert_eq!(list[0].provider, "opencode_go");
    assert_eq!(list[0].category, Some("Qwen".to_string()));

    // Clean up
    std::env::remove_var("CLUTCH_API_BASE_OVERRIDE");
}
