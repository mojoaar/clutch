use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::path::PathBuf;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkspaceEntry {
    pub path: String,
    pub name: String,
    pub project_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
}

/// Tauri command to list all saved workspace directories with their names and detected project types.
#[tauri::command]
pub async fn list_workspaces(pool: State<'_, SqlitePool>) -> Result<Vec<WorkspaceEntry>, String> {
    let row = sqlx::query_scalar::<_, String>("SELECT value FROM settings WHERE key = 'workspaces'")
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    let workspaces: Vec<WorkspaceEntry> = match row {
        Some(json) => serde_json::from_str(&json).unwrap_or_default(),
        None => vec![],
    };

    Ok(workspaces)
}

/// Tauri command to add a new workspace directory after validating it exists and is not a duplicate.
#[tauri::command]
pub async fn add_workspace(
    path: String,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<WorkspaceEntry>, String> {
    let p = PathBuf::from(&path);
    if !p.exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    if !p.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }

    let canonical = p
        .canonicalize()
        .map_err(|e| format!("Failed to resolve path: {}", e))?;
    let resolved = canonical.to_string_lossy().to_string();
    let name = canonical
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| resolved.clone());

    let existing = list_workspaces_inner(pool.inner()).await?;
    if existing.iter().any(|w| w.path == resolved) {
        return Err(format!("Workspace already exists: {}", resolved));
    }

    let mut workspaces = existing;
    workspaces.push(WorkspaceEntry {
        path: resolved,
        name,
        project_type: None,
    });

    save_workspaces(pool.inner(), &workspaces).await?;
    Ok(workspaces)
}

/// Tauri command to remove a workspace directory from the saved list by its path.
#[tauri::command]
pub async fn remove_workspace(
    path: String,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<WorkspaceEntry>, String> {
    let mut workspaces = list_workspaces_inner(pool.inner()).await?;
    workspaces.retain(|w| w.path != path);
    save_workspaces(pool.inner(), &workspaces).await?;
    Ok(workspaces)
}

/// Tauri command to set the active workspace path. Passing None clears the active workspace.
#[tauri::command]
pub async fn set_active_workspace(
    path: Option<String>,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    match path {
        Some(p) => {
            sqlx::query(
                "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('active_workspace', ?, datetime('now'))",
            )
            .bind(&p)
            .execute(pool.inner())
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        }
        None => {
            sqlx::query("DELETE FROM settings WHERE key = 'active_workspace'")
                .execute(pool.inner())
                .await
                .map_err(|e| format!("Database error: {}", e))?;
        }
    }
    Ok(())
}

/// Tauri command to get the currently active workspace path, or None if no workspace is active.
#[tauri::command]
pub async fn get_active_workspace(pool: State<'_, SqlitePool>) -> Result<Option<String>, String> {
    let row = sqlx::query_scalar::<_, String>(
        "SELECT value FROM settings WHERE key = 'active_workspace'",
    )
    .fetch_optional(pool.inner())
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(row)
}

/// Tauri command to scan common development directories under the user's home folder
/// and return detected projects with their inferred project types.
#[tauri::command]
pub async fn detect_workspaces(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<WorkspaceEntry>, String> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| String::from("."));
    let home_path = PathBuf::from(&home);

    let common_names = ["Projects", "Code", "Dev", "Development", "workspace", "workspaces", "src", "repos"];
    let mut detected: Vec<WorkspaceEntry> = Vec::new();
    let existing = list_workspaces_inner(pool.inner()).await?;

    for name in &common_names {
        let dir = home_path.join(name);
        if dir.exists() && dir.is_dir() {
            if let Ok(mut entries) = tokio::fs::read_dir(&dir).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    if path.is_dir() && !path.file_name().map_or(true, |n| n.to_string_lossy().starts_with('.')) {
                        let path_str = path.to_string_lossy().to_string();
                        let dir_name = path.file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        let project_type = detect_project_type(&path);

                        if !existing.iter().any(|w| w.path == path_str)
                            && !detected.iter().any(|w| w.path == path_str)
                        {
                            detected.push(WorkspaceEntry {
                                path: path_str,
                                name: dir_name,
                                project_type,
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(detected)
}

fn detect_project_type(path: &PathBuf) -> Option<String> {
    let indicators: &[(&str, &str)] = &[
        ("package.json", "Node.js"),
        ("Cargo.toml", "Rust"),
        ("pyproject.toml", "Python"),
        ("go.mod", "Go"),
        ("Gemfile", "Ruby"),
        ("pom.xml", "Java/Maven"),
        ("build.gradle", "Java/Gradle"),
        ("build.gradle.kts", "Kotlin/Gradle"),
        ("CMakeLists.txt", "C/C++"),
        ("Makefile", "C/Make"),
        (".git", "Git"),
        ("Dockerfile", "Docker"),
        ("docker-compose.yml", "Docker Compose"),
        ("svelte.config.js", "SvelteKit"),
        ("next.config.js", "Next.js"),
        ("vite.config.ts", "Vite"),
    ];

    for (file, label) in indicators {
        if path.join(file).exists() {
            return Some(label.to_string());
        }
    }
    None
}

/// Tauri command to read the contents of a file within a workspace, enforcing path sandboxing.
#[tauri::command]
pub async fn read_workspace_file(
    workspace_path: String,
    file_path: String,
) -> Result<String, String> {
    let full = PathBuf::from(&workspace_path).join(&file_path);

    let canonical_ws = PathBuf::from(&workspace_path)
        .canonicalize()
        .map_err(|e| format!("Failed to resolve workspace: {}", e))?;
    let canonical_file = full
        .canonicalize()
        .map_err(|e| format!("File not found: {}", e))?;

    if !canonical_file.starts_with(&canonical_ws) {
        return Err("File is outside workspace".to_string());
    }

    tokio::fs::read_to_string(&canonical_file).await
        .map_err(|e| format!("Failed to read file: {}", e))
}

/// Tauri command to write content to a file within a workspace, creating parent directories if needed.
/// Enforces path sandboxing to prevent writes outside the workspace.
#[tauri::command]
pub async fn write_workspace_file(
    workspace_path: String,
    file_path: String,
    content: String,
) -> Result<(), String> {
    let full = PathBuf::from(&workspace_path).join(&file_path);

    let canonical_ws = PathBuf::from(&workspace_path)
        .canonicalize()
        .map_err(|e| format!("Failed to resolve workspace: {}", e))?;

    let full_canonical = match full.canonicalize() {
        Ok(c) => c,
        Err(_) => {
            if let Some(parent) = full.parent() {
                match parent.canonicalize() {
                    Ok(p) => p.join(full.file_name().unwrap_or_default()),
                    Err(e) => return Err(format!("Path resolution failed: {}", e)),
                }
            } else {
                return Err("Invalid path".to_string());
            }
        }
    };

    if full_canonical != full && !full_canonical.starts_with(&canonical_ws) {
        return Err("File is outside workspace".to_string());
    }

    if let Some(parent) = full.parent() {
        if !parent.exists() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| format!("Failed to create parent directories: {}", e))?;
        }
    }

    tokio::fs::write(&full, content).await.map_err(|e| format!("Failed to write file: {}", e))
}

/// Tauri command to list the contents of a directory within a workspace, sorted with directories first.
#[tauri::command]
pub async fn list_workspace_dir(
    workspace_path: String,
    dir_path: String,
) -> Result<Vec<DirEntry>, String> {
    let full = PathBuf::from(&workspace_path).join(&dir_path);

    let canonical_ws = PathBuf::from(&workspace_path)
        .canonicalize()
        .map_err(|e| format!("Failed to resolve workspace: {}", e))?;
    let canonical_dir = full
        .canonicalize()
        .map_err(|e| format!("Directory not found: {}", e))?;

    if !canonical_dir.starts_with(&canonical_ws) {
        return Err("Directory is outside workspace".to_string());
    }

    let mut entries: Vec<DirEntry> = Vec::new();
    let mut dir = tokio::fs::read_dir(&canonical_dir).await
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    while let Some(entry) = dir.next_entry().await
        .map_err(|e| format!("Failed to read directory: {}", e))? {
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.file_type().await.map(|ft| ft.is_dir()).unwrap_or(false);
        let size = entry.metadata().await.map(|m| m.len()).unwrap_or(0);
        entries.push(DirEntry {
            name,
            is_dir,
            size,
        });
    }

    entries.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    Ok(entries)
}

/// Tauri command to create a directory within a workspace, including any missing parent directories.
#[tauri::command]
pub async fn create_workspace_dir(
    workspace_path: String,
    dir_path: String,
) -> Result<(), String> {
    let full = PathBuf::from(&workspace_path).join(&dir_path);

    let canonical_ws = PathBuf::from(&workspace_path)
        .canonicalize()
        .map_err(|e| format!("Failed to resolve workspace: {}", e))?;

    let full_canonical = match full.canonicalize() {
        Ok(c) => c,
        Err(_) => {
            if let Some(parent) = full.parent() {
                match parent.canonicalize() {
                    Ok(p) => p.join(full.file_name().unwrap_or_default()),
                    Err(e) => return Err(format!("Path resolution failed: {}", e)),
                }
            } else {
                return Err("Invalid path".to_string());
            }
        }
    };

    if full_canonical != full && !full_canonical.starts_with(&canonical_ws) {
        return Err("Directory is outside workspace".to_string());
    }

    tokio::fs::create_dir_all(&full).await
        .map_err(|e| format!("Failed to create directory: {}", e))
}

/// Tauri command to delete a directory or file within a workspace.
/// Prevents deletion of the workspace root and enforces path sandboxing.
#[tauri::command]
pub async fn delete_workspace_dir(
    workspace_path: String,
    dir_path: String,
) -> Result<(), String> {
    let full = PathBuf::from(&workspace_path).join(&dir_path);

    let canonical_ws = PathBuf::from(&workspace_path)
        .canonicalize()
        .map_err(|e| format!("Failed to resolve workspace: {}", e))?;
    let canonical_dir = full
        .canonicalize()
        .map_err(|e| format!("Directory not found: {}", e))?;

    if !canonical_dir.starts_with(&canonical_ws) {
        return Err("Directory is outside workspace".to_string());
    }

    if canonical_dir == canonical_ws {
        return Err("Cannot delete workspace root".to_string());
    }

    let metadata = tokio::fs::metadata(&canonical_dir).await
        .map_err(|e| format!("Failed to read metadata: {}", e))?;
    if metadata.is_dir() {
        tokio::fs::remove_dir_all(&canonical_dir).await
            .map_err(|e| format!("Failed to delete directory: {}", e))?;
    } else {
        tokio::fs::remove_file(&canonical_dir).await
            .map_err(|e| format!("Failed to delete file: {}", e))?;
    }

    Ok(())
}

async fn list_workspaces_inner(pool: &SqlitePool) -> Result<Vec<WorkspaceEntry>, String> {
    let row = sqlx::query_scalar::<_, String>("SELECT value FROM settings WHERE key = 'workspaces'")
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    Ok(match row {
        Some(json) => serde_json::from_str(&json).unwrap_or_default(),
        None => vec![],
    })
}

async fn save_workspaces(pool: &SqlitePool, workspaces: &[WorkspaceEntry]) -> Result<(), String> {
    let json = serde_json::to_string(workspaces).map_err(|e| format!("Serialization error: {}", e))?;
    sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES ('workspaces', ?, datetime('now'))",
    )
    .bind(&json)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn detect_empty_directory() {
        let dir = TempDir::new().unwrap();
        assert_eq!(detect_project_type(&dir.path().to_path_buf()), None);
    }

    #[test]
    fn detect_nodejs() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("package.json"), "{}").unwrap();
        assert_eq!(
            detect_project_type(&dir.path().to_path_buf()),
            Some("Node.js".to_string())
        );
    }

    #[test]
    fn detect_rust() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("Cargo.toml"), "").unwrap();
        assert_eq!(
            detect_project_type(&dir.path().to_path_buf()),
            Some("Rust".to_string())
        );
    }

    #[test]
    fn detect_python() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("pyproject.toml"), "").unwrap();
        assert_eq!(
            detect_project_type(&dir.path().to_path_buf()),
            Some("Python".to_string())
        );
    }

    #[test]
    fn detect_go() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("go.mod"), "").unwrap();
        assert_eq!(
            detect_project_type(&dir.path().to_path_buf()),
            Some("Go".to_string())
        );
    }

    #[test]
    fn detect_git() {
        let dir = TempDir::new().unwrap();
        fs::create_dir(dir.path().join(".git")).unwrap();
        assert_eq!(
            detect_project_type(&dir.path().to_path_buf()),
            Some("Git".to_string())
        );
    }

    #[test]
    fn detect_first_match_among_multiple() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("package.json"), "{}").unwrap();
        fs::write(dir.path().join("Cargo.toml"), "").unwrap();
        assert_eq!(
            detect_project_type(&dir.path().to_path_buf()),
            Some("Node.js".to_string())
        );
    }

    #[test]
    fn detect_vite() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("vite.config.ts"), "").unwrap();
        assert_eq!(
            detect_project_type(&dir.path().to_path_buf()),
            Some("Vite".to_string())
        );
    }

    #[test]
    fn detect_sveltekit() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("svelte.config.js"), "").unwrap();
        assert_eq!(
            detect_project_type(&dir.path().to_path_buf()),
            Some("SvelteKit".to_string())
        );
    }

    #[test]
    fn detect_nextjs() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("next.config.js"), "").unwrap();
        assert_eq!(
            detect_project_type(&dir.path().to_path_buf()),
            Some("Next.js".to_string())
        );
    }
}
