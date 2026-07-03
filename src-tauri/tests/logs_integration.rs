use std::path::PathBuf;

#[tokio::test]
async fn get_logs_returns_list_of_log_files() {
    let dir = tempfile::tempdir().unwrap();
    let log_dir = dir.path().join(".clutch").join("logs");
    tokio::fs::create_dir_all(&log_dir).await.unwrap();

    tokio::fs::write(log_dir.join("clutch.log"), "line 1\nline 2").await.unwrap();
    tokio::fs::write(log_dir.join("clutch.log.2026-07-03"), "line 3\nline 4").await.unwrap();

    std::env::set_var("HOME", dir.path());
    std::env::set_var("USERPROFILE", dir.path());

    let response = app_lib::logs::get_logs("".to_string()).await.unwrap();

    assert_eq!(response.files.len(), 2);
    // Sort descending by name: clutch.log.2026-07-03 comes before clutch.log? Wait,cmp b with a, so clutch.log.2026-07-03 comes after clutch.log or before? Let's verify our sort.
    // .sort_by(|a, b| b.name.cmp(&a.name))
    // clutch.log.2026-07-03 starts with c, clutch.log starts with c. clutch.log is shorter. So clutch.log.2026-07-03 is greater. Since b is compared to a, greater comes first, so clutch.log.2026-07-03 comes first.
    assert_eq!(response.files[0].name, "clutch.log.2026-07-03");
    assert_eq!(response.files[1].name, "clutch.log");

    // Default loads newest file (first in the list)
    assert_eq!(response.total_lines, 2);
    assert_eq!(response.lines[0], "line 3");
}

#[tokio::test]
async fn get_logs_retrieves_specific_file() {
    let dir = tempfile::tempdir().unwrap();
    let log_dir = dir.path().join(".clutch").join("logs");
    tokio::fs::create_dir_all(&log_dir).await.unwrap();

    tokio::fs::write(log_dir.join("clutch.log"), "line 1").await.unwrap();
    tokio::fs::write(log_dir.join("clutch.log.2026-07-03"), "specific line").await.unwrap();

    std::env::set_var("HOME", dir.path());
    std::env::set_var("USERPROFILE", dir.path());

    let response = app_lib::logs::get_logs("clutch.log.2026-07-03".to_string()).await.unwrap();
    assert_eq!(response.total_lines, 1);
    assert_eq!(response.lines[0], "specific line");
}

#[tokio::test]
async fn get_logs_empty_directory_graceful() {
    let dir = tempfile::tempdir().unwrap();
    let log_dir = dir.path().join(".clutch").join("logs");
    tokio::fs::create_dir_all(&log_dir).await.unwrap();

    std::env::set_var("HOME", dir.path());
    std::env::set_var("USERPROFILE", dir.path());

    let response = app_lib::logs::get_logs("".to_string()).await.unwrap();
    assert!(response.files.is_empty());
    assert_eq!(response.total_lines, 0);
}

#[tokio::test]
async fn get_logs_traversal_blocked_by_security() {
    let dir = tempfile::tempdir().unwrap();
    let log_dir = dir.path().join(".clutch").join("logs");
    tokio::fs::create_dir_all(&log_dir).await.unwrap();

    // Create a sensitive file outside logs folder
    tokio::fs::write(dir.path().join("secret.txt"), "private info").await.unwrap();

    std::env::set_var("HOME", dir.path());
    std::env::set_var("USERPROFILE", dir.path());

    // Attempting to read ../secret.txt
    let response = app_lib::logs::get_logs("../secret.txt".to_string()).await.unwrap();
    assert_eq!(response.total_lines, 0); // Not found in allowed list, so returns empty
}
