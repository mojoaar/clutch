use std::path::{Path, PathBuf};

/// Detects file path references in text using simple pattern matching for:
/// - Tilde paths: ~/foo/bar.txt
/// - Absolute paths: /Users/name/file.txt
/// - Relative paths: ./src/main.rs or ../../config.json
///
/// Does NOT attempt natural language detection ("read the file X").
/// Skips URLs (containing :// before any path-like content).
/// Returns deduplicated, ordered list of matched paths.
pub fn detect_file_references(text: &str) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut paths = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        // Check for ~/
        if c == '~' && i + 1 < len && chars[i + 1] == '/' {
            let path = extract_path_token(&chars, &mut i);
            if seen.insert(path.clone()) {
                paths.push(path);
            }
            continue;
        }

        // Check for ./ or ../
        if c == '.' && i + 1 < len && chars[i + 1] == '/' {
            let path = extract_path_token(&chars, &mut i);
            if seen.insert(path.clone()) {
                paths.push(path);
            }
            continue;
        }
        if c == '.' && i + 2 < len && chars[i + 1] == '.' && chars[i + 2] == '/' {
            let path = extract_path_token(&chars, &mut i);
            if seen.insert(path.clone()) {
                paths.push(path);
            }
            continue;
        }

        // Check for /absolute/path (but not URLs with ://)
        if c == '/' && i + 1 < len {
            // Walk backward to check if this / is part of a URL (preceded by ://)
            let mut is_url = false;
            if i >= 1 {
                let mut j = i;
                loop {
                    if j >= 2 && chars[j - 2] == ':' && chars[j - 1] == '/' && chars[j] == '/' {
                        is_url = true;
                        break;
                    }
                    if j == 0 || chars[j] == ' ' || chars[j] == '\n' || chars[j] == '\t' {
                        break;
                    }
                    j = j.saturating_sub(1);
                }
            }
            if is_url {
                i += 1;
                continue;
            }
            // Only match if next char is alphanumeric or dot
            let next = chars[i + 1];
            if next.is_alphanumeric() || next == '.' {
                let path = extract_path_token(&chars, &mut i);
                if seen.insert(path.clone()) {
                    paths.push(path);
                }
                continue;
            }
        }

        i += 1;
    }

    paths
}

/// Extracts a path-like token starting at position `i`. Consumes
/// alphanumeric chars, dots, hyphens, slashes, underscores, and backslashes.
/// Advances `i` past the token.
pub fn extract_path_token(chars: &[char], i: &mut usize) -> String {
    let start = *i;
    while *i < chars.len() {
        let c = chars[*i];
        if c.is_alphanumeric() || c == '.' || c == '-' || c == '/' || c == '_' || c == '\\' || c == '~' {
            *i += 1;
        } else {
            break;
        }
    }
    chars[start..*i].iter().collect()
}

/// Checks whether a resolved path is within the allowed security sandbox.
///
/// Allowed:
/// - Active workspace directory
/// - ~/Downloads/
/// - ~/Desktop/
/// - ~/Documents/
///
/// Blocked:
/// - ~/.ssh/, ~/.aws/, ~/.gnupg/ (hidden dot-directories)
/// - /etc/, /var/, /tmp/ (system directories)
/// - Path traversal (../)
pub fn is_path_allowed(path: &Path) -> bool {
    let path_str = path.to_string_lossy();

    if path_str.contains("../") || path_str.contains("..\\") {
        return false;
    }

    let blocked = [
        "/.ssh/", "/.gnupg/", "/.aws/",
        "/Library/Keychains/",
        "/.local/share/keyrings/",
        "/.config/gh/",
        "/etc/", "/var/", "/tmp/",
    ];

    for b in &blocked {
        if path_str.contains(b) {
            return false;
        }
    }

    if let Some(parent) = path.parent() {
        let parent_str = parent.to_string_lossy();
        for component in path_str.split('/') {
            if component.starts_with('.') && component.len() > 1 && !component.starts_with("./") {
                if parent_str.contains("Downloads") || parent_str.contains("Desktop") || parent_str.contains("Documents") {
                    continue;
                }
                return false;
            }
        }
    }

    true
}

/// Resolves a raw path string to an absolute PathBuf.
///
/// - `~` → replaced with HOME env var
/// - Relative paths (`./foo`, `../bar`) → resolved against workspace if provided
/// - Absolute paths → used as-is
pub fn resolve_path(raw: &str, workspace: Option<&str>) -> Result<PathBuf, String> {
    let resolved = if raw.starts_with('~') {
        let home = std::env::var("HOME").map_err(|_| "Could not resolve ~ — no home directory".to_string())?;
        PathBuf::from(home).join(&raw[2..])
    } else if raw.starts_with("./") || raw.starts_with("../") {
        let base = workspace.ok_or_else(|| "Relative path used but no workspace is active".to_string())?;
        let ws = PathBuf::from(base);
        ws.join(raw)
    } else {
        PathBuf::from(raw)
    };

    let canonical = std::fs::canonicalize(&resolved).map_err(|e| format!("Path not found: {} ({})", resolved.display(), e))?;

    if !is_path_allowed(&canonical) {
        return Err(format!("Path blocked by security policy: {}", raw));
    }

    Ok(canonical)
}

/// Reads a file safely:
/// - Max 1MB (truncates to first 500 lines if larger)
/// - Detects binary files by checking first 8KB for null bytes
/// - Returns file content or error message tags
pub async fn read_file_safe(path: &Path) -> Result<String, String> {
    let metadata = tokio::fs::metadata(path).await
        .map_err(|e| format!("Could not read file metadata: {}", e))?;

    if metadata.len() > 1_048_576 {
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| format!("Could not read file: {}", e))?;
        let lines: Vec<&str> = content.lines().take(500).collect();
        return Ok(format!("{}{}", lines.join("\n"), "\n[...truncated at 500 lines]"));
    }

    let buf = tokio::fs::read(path).await
        .map_err(|e| format!("Could not read file: {}", e))?;

    if buf.len() > 8192 {
        let sample = &buf[..8192];
        if sample.contains(&0) {
            return Err("binary — skipped".to_string());
        }
    } else if buf.contains(&0) {
        return Err("binary — skipped".to_string());
    }

    String::from_utf8(buf)
        .map_err(|_| "binary — skipped".to_string())
}

/// Main entry point: detects file path references in the user's message,
/// reads each file, and injects their contents. Returns the enhanced message
/// string with file contents appended, or the original if no files found.
pub async fn inject_file_contents(content: &str, workspace: Option<&str>) -> String {
    let paths = detect_file_references(content);
    if paths.is_empty() {
        return content.to_string();
    }

    let mut result = content.to_string();

    for raw_path in &paths {
        match resolve_path(raw_path, workspace) {
            Ok(resolved) => {
                match read_file_safe(&resolved).await {
                    Ok(file_content) => {
                        result.push_str(&format!("\n\n[File: {}]\n{}", raw_path, file_content));
                    }
                    Err(e) => {
                        result.push_str(&format!("\n\n[File: {} — {}]", raw_path, e));
                    }
                }
            }
            Err(e) => {
                result.push_str(&format!("\n\n[Path: {} — {}]", raw_path, e));
            }
        }
    }

    result
}

/// Tauri command: resolves a path and reads a file, used by the /read slash command.
#[tauri::command]
pub async fn resolve_and_read_file(
    path: String,
    workspace: Option<String>,
) -> Result<String, String> {
    let resolved = resolve_path(&path, workspace.as_deref())?;
    read_file_safe(&resolved).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_tilde_path() {
        let paths = detect_file_references("look at ~/Downloads/report.md");
        assert!(paths.iter().any(|p| p.contains("~/Downloads/report.md")));
    }

    #[test]
    fn test_detect_absolute_path() {
        let paths = detect_file_references("check /Users/test/file.txt");
        assert!(paths.iter().any(|p| p.contains("/Users/test/file.txt")));
    }

    #[test]
    fn test_detect_relative_path() {
        let paths = detect_file_references("see ./src/main.rs for details");
        assert!(paths.iter().any(|p| p.contains("./src/main.rs")));
    }

    #[test]
    fn test_detect_parent_path() {
        let paths = detect_file_references("../../config.json has settings");
        assert!(paths.iter().any(|p| p.contains("../../config.json")));
    }

    #[test]
    fn test_no_path_in_url() {
        let paths = detect_file_references("visit https://example.com/path/to/page");
        assert!(!paths.iter().any(|p| p.contains("/path/to/page")));
    }

    #[test]
    fn test_multiple_paths_detected() {
        let paths = detect_file_references("read ~/a.txt and ./b.md");
        assert!(paths.len() >= 2);
    }

    #[test]
    fn test_block_ssh_dir() {
        let ssh = Path::new("/home/user/.ssh/id_rsa");
        assert!(!is_path_allowed(ssh));
    }

    #[test]
    fn test_block_dotfile() {
        let dot = Path::new("/home/user/.secret/config");
        assert!(!is_path_allowed(dot));
    }

    #[test]
    fn test_block_etc_dir() {
        let etc = Path::new("/etc/passwd");
        assert!(!is_path_allowed(etc));
    }

    #[test]
    fn test_allow_downloads() {
        let dl = Path::new("/home/user/Downloads/report.md");
        assert!(is_path_allowed(dl));
    }

    #[test]
    fn test_block_traversal() {
        let trav = Path::new("foo/../../etc/passwd");
        assert!(!is_path_allowed(trav));
    }

    #[test]
    fn test_empty_text_no_paths() {
        let paths = detect_file_references("");
        assert!(paths.is_empty());
    }
}
