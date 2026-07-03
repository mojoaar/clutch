mod common;

use common::create_test_pool;
use tauri::Manager;
use app_lib::skills::{get_skill_instructions, execute_skill_action, SkillAction};

#[tokio::test]
async fn get_skill_instructions_reads_disk_successfully() {
    let dir = tempfile::tempdir().unwrap();
    let skill_md_dir = dir.path().join(".clutch").join("skills").join("my-skill");
    tokio::fs::create_dir_all(&skill_md_dir).await.unwrap();
    tokio::fs::write(skill_md_dir.join("SKILL.md"), "instructions details").await.unwrap();

    std::env::set_var("HOME", dir.path());
    std::env::set_var("USERPROFILE", dir.path());

    let inst = get_skill_instructions("my-skill".to_string()).await.unwrap();
    assert_eq!(inst, "instructions details");
}

#[tokio::test]
async fn execute_skill_action_read_file_within_sandbox() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let home_dir = tempfile::tempdir().unwrap();
    let skill_dir = home_dir.path().join(".clutch").join("skills").join("test-skill");
    tokio::fs::create_dir_all(&skill_dir).await.unwrap();
    tokio::fs::write(skill_dir.join("SKILL.md"), "instructions").await.unwrap();

    let ws_dir = tempfile::tempdir().unwrap();
    let ws_path = ws_dir.path().to_string_lossy().to_string();

    tokio::fs::write(ws_dir.path().join("source.rs"), "fn main() {}").await.unwrap();

    std::env::set_var("HOME", home_dir.path());
    std::env::set_var("USERPROFILE", home_dir.path());

    let action = SkillAction {
        action: "read_file".to_string(),
        path: "source.rs".to_string(),
        content: None,
        command: None,
    };

    let read_content = execute_skill_action(app.state(), "test-skill".to_string(), action, Some(ws_path)).await.unwrap();
    assert_eq!(read_content, "fn main() {}");
}

#[tokio::test]
async fn execute_skill_action_write_file_within_sandbox() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let home_dir = tempfile::tempdir().unwrap();
    let skill_dir = home_dir.path().join(".clutch").join("skills").join("test-skill");
    tokio::fs::create_dir_all(&skill_dir).await.unwrap();
    tokio::fs::write(skill_dir.join("SKILL.md"), "instructions").await.unwrap();

    let ws_dir = tempfile::tempdir().unwrap();
    let ws_path = ws_dir.path().to_string_lossy().to_string();

    tokio::fs::create_dir(ws_dir.path().join("sub")).await.unwrap();

    std::env::set_var("HOME", home_dir.path());
    std::env::set_var("USERPROFILE", home_dir.path());

    let action = SkillAction {
        action: "write_file".to_string(),
        path: "sub/out.txt".to_string(),
        content: Some("written content".to_string()),
        command: None,
    };

    let res = execute_skill_action(app.state(), "test-skill".to_string(), action, Some(ws_path)).await.unwrap();
    assert_eq!(res, "Wrote to sub/out.txt");

    let content = tokio::fs::read_to_string(ws_dir.path().join("sub").join("out.txt")).await.unwrap();
    assert_eq!(content, "written content");
}

#[tokio::test]
async fn execute_skill_action_list_dir_within_sandbox() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let home_dir = tempfile::tempdir().unwrap();
    let skill_dir = home_dir.path().join(".clutch").join("skills").join("test-skill");
    tokio::fs::create_dir_all(&skill_dir).await.unwrap();
    tokio::fs::write(skill_dir.join("SKILL.md"), "instructions").await.unwrap();

    let ws_dir = tempfile::tempdir().unwrap();
    let ws_path = ws_dir.path().to_string_lossy().to_string();

    tokio::fs::write(ws_dir.path().join("a.txt"), "").await.unwrap();
    tokio::fs::create_dir(ws_dir.path().join("b_dir")).await.unwrap();

    std::env::set_var("HOME", home_dir.path());
    std::env::set_var("USERPROFILE", home_dir.path());

    let action = SkillAction {
        action: "list_dir".to_string(),
        path: "".to_string(),
        content: None,
        command: None,
    };

    let list = execute_skill_action(app.state(), "test-skill".to_string(), action, Some(ws_path)).await.unwrap();
    assert!(list.contains("[FILE] a.txt"));
    assert!(list.contains("[DIR]  b_dir"));
}

#[tokio::test]
async fn execute_skill_action_outside_sandbox_denied() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let home_dir = tempfile::tempdir().unwrap();
    let skill_dir = home_dir.path().join(".clutch").join("skills").join("test-skill");
    tokio::fs::create_dir_all(&skill_dir).await.unwrap();
    tokio::fs::write(skill_dir.join("SKILL.md"), "instructions").await.unwrap();

    let ws_dir = tempfile::tempdir().unwrap();
    let ws_path = ws_dir.path().to_string_lossy().to_string();

    // Create outside file on disk, but in a sibling folder (which exists, but falls outside the workspace)
    let sibling_dir = tempfile::tempdir().unwrap();
    let outside_file = sibling_dir.path().join("outside.txt");
    tokio::fs::write(&outside_file, "secrets").await.unwrap();
    let outside_file_str = outside_file.to_string_lossy().to_string();

    std::env::set_var("HOME", home_dir.path());
    std::env::set_var("USERPROFILE", home_dir.path());

    let action = SkillAction {
        action: "read_file".to_string(),
        path: outside_file_str,
        content: None,
        command: None,
    };

    let res = execute_skill_action(app.state(), "test-skill".to_string(), action, Some(ws_path)).await;
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("Access denied"));
}
