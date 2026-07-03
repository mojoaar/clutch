use app_lib::workspaces::{
    read_workspace_file, write_workspace_file, list_workspace_dir,
    create_workspace_dir, delete_workspace_dir
};

#[tokio::test]
async fn read_workspace_file_inside_sandbox() {
    let ws = tempfile::tempdir().unwrap();
    let ws_path_str = ws.path().to_string_lossy().to_string();

    tokio::fs::write(ws.path().join("foo.txt"), "hello workspace").await.unwrap();

    let content = read_workspace_file(ws_path_str, "foo.txt".to_string()).await.unwrap();
    assert_eq!(content, "hello workspace");
}

#[tokio::test]
async fn read_workspace_file_outside_sandbox_fails() {
    let ws = tempfile::tempdir().unwrap();
    let ws_path_str = ws.path().to_string_lossy().to_string();

    // Sensitive file outside workspace
    let out_dir = tempfile::tempdir().unwrap();
    let out_file = out_dir.path().join("secret.txt");
    tokio::fs::write(&out_file, "secrets").await.unwrap();

    let out_file_str = out_file.to_string_lossy().to_string();

    let res = read_workspace_file(ws_path_str, out_file_str).await;
    assert!(res.is_err());
    let err_msg = res.unwrap_err();
    assert!(err_msg.contains("outside workspace") || err_msg.contains("not found"));
}

#[tokio::test]
async fn write_workspace_file_inside_sandbox() {
    let ws = tempfile::tempdir().unwrap();
    let ws_path_str = ws.path().to_string_lossy().to_string();

    tokio::fs::create_dir(ws.path().join("nested")).await.unwrap();
    write_workspace_file(ws_path_str, "nested/bar.txt".to_string(), "hello write".to_string()).await.unwrap();

    let written = tokio::fs::read_to_string(ws.path().join("nested").join("bar.txt")).await.unwrap();
    assert_eq!(written, "hello write");
}

#[tokio::test]
async fn write_workspace_file_outside_sandbox_fails() {
    let ws = tempfile::tempdir().unwrap();
    let ws_path_str = ws.path().to_string_lossy().to_string();

    let res = write_workspace_file(ws_path_str, "../hacked.txt".to_string(), "hack".to_string()).await;
    assert!(res.is_err());
    assert!(res.unwrap_err().contains("outside workspace"));
}

#[tokio::test]
async fn list_workspace_dir_inside_sandbox() {
    let ws = tempfile::tempdir().unwrap();
    let ws_path_str = ws.path().to_string_lossy().to_string();

    tokio::fs::write(ws.path().join("file.txt"), "file").await.unwrap();
    tokio::fs::create_dir(ws.path().join("folder")).await.unwrap();

    let list = list_workspace_dir(ws_path_str, "".to_string()).await.unwrap();
    assert_eq!(list.len(), 2);
    // directories first
    assert!(list[0].is_dir);
    assert_eq!(list[0].name, "folder");
    assert!(!list[1].is_dir);
    assert_eq!(list[1].name, "file.txt");
}

#[tokio::test]
async fn create_workspace_dir_inside_sandbox() {
    let ws = tempfile::tempdir().unwrap();
    let ws_path_str = ws.path().to_string_lossy().to_string();

    tokio::fs::create_dir(ws.path().join("sub")).await.unwrap();
    create_workspace_dir(ws_path_str, "sub/dir".to_string()).await.unwrap();
    assert!(ws.path().join("sub").join("dir").is_dir());
}

#[tokio::test]
async fn delete_workspace_dir_inside_sandbox() {
    let ws = tempfile::tempdir().unwrap();
    let ws_path_str = ws.path().to_string_lossy().to_string();

    let target_file = ws.path().join("temp.txt");
    tokio::fs::write(&target_file, "to delete").await.unwrap();

    delete_workspace_dir(ws_path_str, "temp.txt".to_string()).await.unwrap();
    assert!(!target_file.exists());
}
