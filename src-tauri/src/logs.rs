use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogFile {
    pub name: String,
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogResponse {
    pub files: Vec<LogFile>,
    pub lines: Vec<String>,
    pub total_lines: usize,
}

/// Tauri command to retrieve application log files and their contents from ~/.clutch/logs.
/// Returns the list of available log files along with the lines of the requested log file.
#[tauri::command]
pub async fn get_logs(file: String) -> Result<LogResponse, String> {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .map_err(|e| e.to_string())?;
    let log_dir = std::path::PathBuf::from(home).join(".clutch").join("logs");

    let mut files = Vec::new();
    if let Ok(mut entries) = tokio::fs::read_dir(&log_dir).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            if name.starts_with("clutch.log") {
                if let Ok(meta) = path.metadata() {
                    files.push(LogFile {
                        name,
                        path: path.to_string_lossy().to_string(),
                        size: meta.len(),
                    });
                }
            }
        }
    }
    files.sort_by(|a, b| b.name.cmp(&a.name));

    let target = if file.is_empty() {
        files.first().map(|f| f.path.clone()).unwrap_or_default()
    } else {
        let basename = std::path::Path::new(&file)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        files.iter()
            .find(|f| f.name == basename)
            .map(|f| f.path.clone())
            .unwrap_or_default()
    };

    if target.is_empty() {
        return Ok(LogResponse {
            files,
            lines: vec![],
            total_lines: 0,
        });
    }

    let content = tokio::fs::read_to_string(&target).await
        .map_err(|e| format!("Cannot read log file: {}", e))?;
    let all_lines: Vec<String> = content.lines().map(String::from).collect();
    let total_lines = all_lines.len();

    Ok(LogResponse {
        files,
        lines: all_lines,
        total_lines,
    })
}
