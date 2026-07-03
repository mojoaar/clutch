mod common;

use common::create_test_pool;
use tauri::Manager;
use app_lib::export::{export_session, ExportOptions};

#[tokio::test]
async fn export_session_not_found_returns_err() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let opts = ExportOptions {
        format: "markdown".to_string(),
        include_metadata: true,
        include_timestamps: true,
        include_provider_info: true,
        include_system_prompt: true,
    };

    let res = export_session(app.state(), "missing-id".to_string(), opts).await;
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "Session not found");
}

#[tokio::test]
async fn export_session_unsupported_format_errors() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    // Create session
    app_lib::sessions::create_session(
        app.state(), "s1".into(), "Title".into(), "deepseek".into(), "deepseek-v4-pro".into(), None
    ).await.unwrap();

    let opts = ExportOptions {
        format: "pdf".to_string(),
        include_metadata: true,
        include_timestamps: true,
        include_provider_info: true,
        include_system_prompt: true,
    };

    let res = export_session(app.state(), "s1".to_string(), opts).await;
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Unsupported format"));
}

#[tokio::test]
async fn export_session_markdown_rendering() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    // Create session and messages
    app_lib::sessions::create_session(
        app.state(), "s1".into(), "Chat title".into(), "deepseek".into(), "deepseek-v4-pro".into(), Some("Sys Prompt".into())
    ).await.unwrap();
    app_lib::sessions::create_message(
        app.state(), "m1".into(), "s1".into(), "user".into(), "Hello assistant".into(), Some(10)
    ).await.unwrap();
    app_lib::sessions::create_message(
        app.state(), "m2".into(), "s1".into(), "assistant".into(), "Hello user".into(), Some(20)
    ).await.unwrap();

    let opts = ExportOptions {
        format: "markdown".to_string(),
        include_metadata: true,
        include_timestamps: true,
        include_provider_info: true,
        include_system_prompt: true,
    };

    let exported = export_session(app.state(), "s1".to_string(), opts).await.unwrap();
    assert!(exported.contains("# Chat title"));
    assert!(exported.contains("Sys Prompt"));
    assert!(exported.contains("**You**"));
    assert!(exported.contains("Hello assistant"));
    assert!(exported.contains("**Assistant**"));
    assert!(exported.contains("Hello user"));
}

#[tokio::test]
async fn export_session_plain_text() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    app_lib::sessions::create_session(
        app.state(), "s2".into(), "Text title".into(), "deepseek".into(), "deepseek-v4-pro".into(), None
    ).await.unwrap();
    app_lib::sessions::create_message(
        app.state(), "m1".into(), "s2".into(), "user".into(), "Raw content".into(), Some(0)
    ).await.unwrap();

    let opts = ExportOptions {
        format: "text".to_string(),
        include_metadata: false,
        include_timestamps: false,
        include_provider_info: false,
        include_system_prompt: false,
    };

    let exported = export_session(app.state(), "s2".to_string(), opts).await.unwrap();
    assert!(exported.contains("You"));
    assert!(exported.contains("Raw content"));
    assert!(exported.contains("Text title"));
}

#[tokio::test]
async fn export_session_json() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    app_lib::sessions::create_session(
        app.state(), "s3".into(), "JSON title".into(), "deepseek".into(), "deepseek-v4-pro".into(), None
    ).await.unwrap();

    let opts = ExportOptions {
        format: "json".to_string(),
        include_metadata: true,
        include_timestamps: true,
        include_provider_info: true,
        include_system_prompt: true,
    };

    let exported = export_session(app.state(), "s3".to_string(), opts).await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&exported).unwrap();
    assert_eq!(json["session"]["id"], "s3");
    assert_eq!(json["session"]["title"], "JSON title");
    assert_eq!(json["messages"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn export_session_html() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    app_lib::sessions::create_session(
        app.state(), "s4".into(), "HTML title".into(), "deepseek".into(), "deepseek-v4-pro".into(), None
    ).await.unwrap();
    app_lib::sessions::create_message(
        app.state(), "m1".into(), "s4".into(), "user".into(), "Markup".into(), Some(0)
    ).await.unwrap();

    let opts = ExportOptions {
        format: "html".to_string(),
        include_metadata: true,
        include_timestamps: true,
        include_provider_info: true,
        include_system_prompt: true,
    };

    let exported = export_session(app.state(), "s4".to_string(), opts).await.unwrap();
    assert!(exported.contains("<!DOCTYPE html>"));
    assert!(exported.contains("HTML title"));
    assert!(exported.contains("Markup"));
}
