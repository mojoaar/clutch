mod common;

use common::create_test_pool;
use tauri::Manager;
use app_lib::skills::{get_skill_detail, install_skill, check_skill_updates, list_installed_skills};

static SERIAL_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[tokio::test]
async fn get_skill_detail_with_mock_github() {
    let _guard = SERIAL_LOCK.lock().unwrap();
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let mut server = mockito::Server::new_async().await;
    std::env::set_var("CLUTCH_GITHUB_RAW_OVERRIDE", server.url());

    let skill_md = r#"---
name: Net Skill
description: Tested with mock server
---
# Instructions
Do great network calls."#;

    let mock = server.mock("GET", "/anthropics/skills/main/net-skill/SKILL.md")
        .with_status(200)
        .with_body(skill_md)
        .create_async()
        .await;

    let detail = get_skill_detail("net-skill".to_string(), "anthropics/skills".to_string(), "main".to_string(), app.state()).await.unwrap();
    mock.assert_async().await;

    assert_eq!(detail.name, "Net Skill");
    assert_eq!(detail.description, "Tested with mock server");
    assert_eq!(detail.instructions, skill_md);

    std::env::remove_var("CLUTCH_GITHUB_RAW_OVERRIDE");
}

#[tokio::test]
async fn install_skill_downloads_and_saves() {
    let _guard = SERIAL_LOCK.lock().unwrap();
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let home_dir = tempfile::tempdir().unwrap();
    std::env::set_var("HOME", home_dir.path());
    std::env::set_var("USERPROFILE", home_dir.path());

    let mut server = mockito::Server::new_async().await;
    std::env::set_var("CLUTCH_GITHUB_RAW_OVERRIDE", server.url());

    let skill_md = r#"---
name: Downloaded Skill
description: Installed from mock GitHub
---
# Instructions"#;

    let mock = server.mock("GET", "/anthropics/skills/main/downloaded-skill/SKILL.md")
        .with_status(200)
        .with_body(skill_md)
        .create_async()
        .await;

    // Call install
    install_skill("downloaded-skill".to_string(), "anthropics/skills".to_string(), "main".to_string(), app.state()).await.unwrap();
    mock.assert_async().await;

    // Verify written to disk under temp home dir
    let inst_file = home_dir.path().join(".clutch").join("skills").join("downloaded-skill").join("SKILL.md");
    assert!(inst_file.exists());
    let written = tokio::fs::read_to_string(inst_file).await.unwrap();
    assert!(written.contains("Downloaded Skill"));

    // Verify DB settings inserted
    let list = list_installed_skills(app.state()).await.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, "downloaded-skill");
    assert_eq!(list[0].name, "Downloaded Skill");

    std::env::remove_var("CLUTCH_GITHUB_RAW_OVERRIDE");
}

#[tokio::test]
async fn check_skill_updates_with_mock_github_api() {
    let _guard = SERIAL_LOCK.lock().unwrap();
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let mut server = mockito::Server::new_async().await;
    std::env::set_var("CLUTCH_GITHUB_API_OVERRIDE", server.url());

    // Register a mock skill in DB with a custom current_version (old sha)
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('skill:anthropics/skills/my-skill:installed', 'true')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('skill:anthropics/skills/my-skill:name', 'My Skill')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('skill:anthropics/skills/my-skill:source', 'anthropics/skills')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('skill:anthropics/skills/my-skill:version', 'abc1234')")
        .execute(&pool).await.unwrap();

    let commits_json = r#"{"sha": "def567890abcdef"}"#;
    let mock = server.mock("GET", "/repos/anthropics/skills/commits/main")
        .with_status(200)
        .with_body(commits_json)
        .create_async()
        .await;

    let updates = check_skill_updates(app.state()).await.unwrap();
    mock.assert_async().await;

    assert_eq!(updates.len(), 1);
    assert_eq!(updates[0].name, "My Skill");
    assert!(updates[0].has_update);
    assert_eq!(updates[0].current_version.as_deref(), Some("abc1234"));
    assert_eq!(updates[0].latest_version, "def5678"); // First 7 chars

    std::env::remove_var("CLUTCH_GITHUB_API_OVERRIDE");
}
