mod common;

use common::create_test_pool;
use tauri::Manager;
use app_lib::skills::{list_installed_skills, parse_skill_md};

#[test]
fn parse_skill_md_extracts_all_fields() {
    let md = r#"---
name: Git Helper
description: Automatically formats commit messages following guidelines.
---
# Instructions
Always write clear commits."#;

    let (name, desc, _) = parse_skill_md(md);
    assert_eq!(name, "Git Helper");
    assert_eq!(desc, "Automatically formats commit messages following guidelines.");
}

#[test]
fn parse_skill_md_missing_frontmatter_defaults_to_title() {
    let md = r#"# Instructions
No frontmatter here."#;

    let (name, desc, _) = parse_skill_md(md);
    assert_eq!(name, "Instructions");
    assert_eq!(desc, "");
}

#[test]
fn parse_skill_md_malformed_frontmatter_graceful() {
    let md = r#"---
invalid-yaml-lines
---
# Instructions"#;

    let (name, desc, _) = parse_skill_md(md);
    assert_eq!(name, "Instructions");
    assert_eq!(desc, "");
}

#[tokio::test]
async fn list_installed_skills_empty_by_default() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    let list = list_installed_skills(app.state()).await.unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_installed_skills_retrieves_from_db() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    // Insert skill keys directly
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('skill:git:installed', 'true')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('skill:git:name', 'Git Helper')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('skill:git:desc', 'Helper for git commits')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('skill:git:source', 'bundled')")
        .execute(&pool).await.unwrap();

    let list = list_installed_skills(app.state()).await.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, "git");
    assert_eq!(list[0].name, "Git Helper");
    assert_eq!(list[0].description, "Helper for git commits");
    assert_eq!(list[0].source, "bundled");
    assert!(list[0].installed);
}

#[tokio::test]
async fn uninstall_skill_removes_settings() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());

    // Insert skill keys directly
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('skill:foo:installed', 'true')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('skill:foo:name', 'Foo Skill')")
        .execute(&pool).await.unwrap();
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES ('skill:foo:desc', 'Foo desc')")
        .execute(&pool).await.unwrap();

    // Call uninstall
    app_lib::skills::uninstall_skill("foo".to_string(), app.state()).await.unwrap();

    let list = list_installed_skills(app.state()).await.unwrap();
    assert!(list.is_empty());

    // Verify DB rows deleted
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM settings WHERE key LIKE 'skill:foo:%'")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(count.0, 0);
}
