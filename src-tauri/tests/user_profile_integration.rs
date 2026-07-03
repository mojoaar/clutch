mod common;

use common::create_test_pool;
use tauri::Manager;

#[tokio::test]
async fn get_profile_returns_default_row_initially() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    
    // Migration inserts default 'User' profile
    let profile = app_lib::user_profile::get_user_profile(app.state()).await.unwrap().unwrap();
    assert_eq!(profile.display_name, "User");
    assert_eq!(profile.avatar_type, "icon");
}

#[tokio::test]
async fn update_profile_inserts_new_row() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    app_lib::user_profile::update_user_profile(
        app.state(),
        "Mojo Aar".to_string(),
        "emoji".to_string(),
        "👋".to_string(),
        "#ff0000".to_string(),
    )
    .await
    .unwrap();

    let profile = app_lib::user_profile::get_user_profile(app.state()).await.unwrap().unwrap();
    assert_eq!(profile.display_name, "Mojo Aar");
    assert_eq!(profile.avatar_type, "emoji");
    assert_eq!(profile.avatar_data, "👋");
    assert_eq!(profile.avatar_color, "#ff0000");
}

#[tokio::test]
async fn update_profile_overwrites_existing_row() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    
    app_lib::user_profile::update_user_profile(
        app.state(),
        "Old Name".to_string(),
        "emoji".to_string(),
        "👴".to_string(),
        "#000000".to_string(),
    )
    .await
    .unwrap();

    app_lib::user_profile::update_user_profile(
        app.state(),
        "New Name".to_string(),
        "initials".to_string(),
        "NN".to_string(),
        "#ffffff".to_string(),
    )
    .await
    .unwrap();

    let profile = app_lib::user_profile::get_user_profile(app.state()).await.unwrap().unwrap();
    assert_eq!(profile.display_name, "New Name");
    assert_eq!(profile.avatar_type, "initials");
    assert_eq!(profile.avatar_data, "NN");
    assert_eq!(profile.avatar_color, "#ffffff");
}

#[tokio::test]
async fn update_profile_respects_avatar_type_check_constraint() {
    let (pool, _dir) = create_test_pool().await;
    let app = tauri::test::mock_app();
    app.manage(pool.clone());
    
    // Passing invalid avatar_type should error
    let res = app_lib::user_profile::update_user_profile(
        app.state(),
        "Test".to_string(),
        "invalid_type".to_string(),
        "".to_string(),
        "".to_string(),
    )
    .await;
    
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("CHECK constraint failed"));
}
