mod common;

use common::create_test_pool;
use tauri::Manager;
use app_lib::workspaces::{
    list_workspaces, add_workspace, remove_workspace, set_active_workspace,
    get_active_workspace, detect_workspaces
};

#[tokio::test]
async fn list_workspaces_empty_initially() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let list = list_workspaces(app.state()).await.unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_workspaces_with_inserted_json() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let json = r#"[
        {"path":"/Users/mojo/project-a","name":"project-a","project_type":"Rust"},
        {"path":"/Users/mojo/project-b","name":"project-b","project_type":"Node"}
    ]"#;

    sqlx::query("INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('workspaces', ?, datetime('now'))")
        .bind(json)
        .execute(&pool)
        .await
        .unwrap();

    let list = list_workspaces(app.state()).await.unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].name, "project-a");
    assert_eq!(list[0].path, "/Users/mojo/project-a");
    assert_eq!(list[1].name, "project-b");
    assert_eq!(list[1].path, "/Users/mojo/project-b");
}

#[tokio::test]
async fn add_workspace_nonexistent_fails() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let res = add_workspace("/nonexistent/path/on/disk".to_string(), app.state()).await;
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Path does not exist"));
}

#[tokio::test]
async fn add_workspace_existing_dir_adds_successfully() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let temp_workspace = tempfile::tempdir().unwrap();
    let path_str = temp_workspace.path().to_string_lossy().to_string();

    let list = add_workspace(path_str.clone(), app.state()).await.unwrap();
    assert_eq!(list.len(), 1);
    
    // Canonicalized path check (MacOS prepends /private)
    let added_path = &list[0].path;
    let canonical = temp_workspace.path().canonicalize().unwrap().to_string_lossy().to_string();
    assert_eq!(added_path, &canonical);

    // Verify persisted in DB
    let db_json: (String,) = sqlx::query_as("SELECT value FROM settings WHERE key = 'workspaces'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(db_json.0.contains(added_path));
}

#[tokio::test]
async fn add_workspace_duplicate_fails() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let temp_workspace = tempfile::tempdir().unwrap();
    let path_str = temp_workspace.path().to_string_lossy().to_string();

    // First time
    add_workspace(path_str.clone(), app.state()).await.unwrap();

    // Duplicate
    let res = add_workspace(path_str.clone(), app.state()).await;
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Workspace already exists"));
}

#[tokio::test]
async fn remove_workspace_removes_correct_path() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let json = r#"[
        {"path":"/path/a","name":"a"},
        {"path":"/path/b","name":"b"}
    ]"#;

    sqlx::query("INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('workspaces', ?, datetime('now'))")
        .bind(json)
        .execute(&pool)
        .await
        .unwrap();

    let list = remove_workspace("/path/a".to_string(), app.state()).await.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].path, "/path/b");

    // Verify DB updated
    let db_json: (String,) = sqlx::query_as("SELECT value FROM settings WHERE key = 'workspaces'")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(!db_json.0.contains("/path/a"));
}

#[tokio::test]
async fn set_and_get_active_workspace() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    set_active_workspace(Some("/path/active".to_string()), app.state()).await.unwrap();
    let active = get_active_workspace(app.state()).await.unwrap();
    assert_eq!(active, Some("/path/active".to_string()));

    // Clear active
    set_active_workspace(None, app.state()).await.unwrap();
    let active = get_active_workspace(app.state()).await.unwrap();
    assert_eq!(active, None);
}

#[tokio::test]
async fn detect_workspaces_with_temp_home_directory() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let temp_home = tempfile::tempdir().unwrap();
    let dev_dir = temp_home.path().join("Development");
    std::fs::create_dir_all(&dev_dir).unwrap();

    // Create a mock rust project inside temp_home/Development/rust-app/Cargo.toml
    let rust_proj = dev_dir.join("rust-app");
    std::fs::create_dir_all(&rust_proj).unwrap();
    std::fs::write(rust_proj.join("Cargo.toml"), "[package]\nname = \"rust-app\"").unwrap();

    // Create a mock node project inside temp_home/Development/node-app/package.json
    let node_proj = dev_dir.join("node-app");
    std::fs::create_dir_all(&node_proj).unwrap();
    std::fs::write(node_proj.join("package.json"), "{\"name\": \"node-app\"}").unwrap();

    // Isolate environment to our temp home
    std::env::set_var("HOME", temp_home.path());
    std::env::set_var("USERPROFILE", temp_home.path());

    let detected = detect_workspaces(app.state()).await.unwrap();
    assert_eq!(detected.len(), 2);

    let rust_entry = detected.iter().find(|w| w.name == "rust-app").unwrap();
    assert_eq!(rust_entry.project_type.as_deref(), Some("Rust"));

    let node_entry = detected.iter().find(|w| w.name == "node-app").unwrap();
    assert_eq!(node_entry.project_type.as_deref(), Some("Node.js"));
}
