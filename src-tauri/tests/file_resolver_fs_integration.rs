use std::path::{Path, PathBuf};
use app_lib::file_resolver::{
    resolve_path, read_file_safe, inject_file_contents, resolve_and_read_file
};

#[tokio::test]
async fn resolve_path_tilde_expansion() {
    let home_dir = tempfile::tempdir().unwrap();
    std::env::set_var("HOME", home_dir.path());
    std::env::set_var("USERPROFILE", home_dir.path());

    // Create a real file on disk so canonicalize() passes
    let file = home_dir.path().join("test.txt");
    tokio::fs::write(&file, "content").await.unwrap();

    let resolved = resolve_path("~/test.txt", None).unwrap();
    assert_eq!(resolved, file.canonicalize().unwrap());
}

#[tokio::test]
async fn resolve_path_relative_no_workspace_fails() {
    let res = resolve_path("./relative.txt", None);
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("no workspace is active"));
}

#[tokio::test]
async fn resolve_path_relative_with_workspace() {
    let ws = tempfile::tempdir().unwrap();
    let file = ws.path().join("foo.txt");
    tokio::fs::write(&file, "foo").await.unwrap();

    let resolved = resolve_path("./foo.txt", Some(&ws.path().to_string_lossy())).unwrap();
    assert_eq!(resolved, file.canonicalize().unwrap());
}

#[tokio::test]
async fn read_file_safe_happy_path() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("plain.txt");
    tokio::fs::write(&file, "hello plain").await.unwrap();

    let content = read_file_safe(&file).await.unwrap();
    assert_eq!(content, "hello plain");
}

#[tokio::test]
async fn read_file_safe_binary_is_skipped() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("binary.bin");
    tokio::fs::write(&file, b"hello\0world").await.unwrap();

    let res = read_file_safe(&file).await;
    assert!(res.is_err());
    assert_eq!(res.unwrap_err(), "binary — skipped");
}

#[tokio::test]
async fn read_file_safe_large_file_truncates() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("large.txt");
    
    let mut large_content = String::new();
    for i in 1..1000 {
        large_content.push_str(&format!("Line {}\n", i));
    }
    tokio::fs::write(&file, &large_content).await.unwrap();

    // Verify it is larger than 1MB (Wait, 1000 lines is only 10KB, let's write an actual large string or just mock write size to trigger len > 1MB)
    // Actually, line 182 in file_resolver.rs triggers if metadata.len() > 1_048_576. Let's make it larger!
    let mut nested_content = large_content.clone();
    while nested_content.len() <= 1_048_576 {
        nested_content.push_str(&large_content);
    }
    tokio::fs::write(&file, &nested_content).await.unwrap();

    let content = read_file_safe(&file).await.unwrap();
    assert!(content.contains("[...truncated at 500 lines]"));
    // Count lines, should be 501 (500 lines + truncation notice)
    assert_eq!(content.lines().count(), 501);
}

#[tokio::test]
async fn inject_file_contents_none_found() {
    let text = "just simple chat with no paths";
    let injected = inject_file_contents(text, None).await;
    assert_eq!(injected, text);
}

#[tokio::test]
async fn inject_file_contents_happy_path() {
    let ws = tempfile::tempdir().unwrap();
    let file = ws.path().join("code.py");
    tokio::fs::write(&file, "print('hello')").await.unwrap();

    let prompt = "review my code at ./code.py please";
    let injected = inject_file_contents(prompt, Some(&ws.path().to_string_lossy())).await;
    
    assert!(injected.contains("review my code"));
    assert!(injected.contains("[File: ./code.py]"));
    assert!(injected.contains("print('hello')"));
}

#[tokio::test]
async fn inject_file_contents_missing_file_fallback() {
    let ws = tempfile::tempdir().unwrap();
    let prompt = "review my code at ./missing.rs please";
    let injected = inject_file_contents(prompt, Some(&ws.path().to_string_lossy())).await;
    
    assert!(injected.contains("[Path: ./missing.rs — Path not found:"));
}

#[tokio::test]
async fn resolve_and_read_file_command_happy_path() {
    let ws = tempfile::tempdir().unwrap();
    let file = ws.path().join("foo.txt");
    tokio::fs::write(&file, "bar").await.unwrap();

    let content = resolve_and_read_file("./foo.txt".to_string(), Some(ws.path().to_string_lossy().to_string())).await.unwrap();
    assert_eq!(content, "bar");
}
