use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillDetail {
    pub id: String,
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub source: String,
    pub branch: String,
    pub installed: bool,
    pub install_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillAction {
    pub action: String,
    pub path: String,
    pub content: Option<String>,
    pub command: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SkillUpdateInfo {
    pub id: String,
    pub name: String,
    pub has_update: bool,
    pub current_version: Option<String>,
    pub latest_version: String,
}

fn skills_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| String::from("."));
    PathBuf::from(home).join(".clutch").join("skills")
}

/// Tauri command to fetch the full details of a skill by fetching its SKILL.md from GitHub.
#[tauri::command]
pub async fn get_skill_detail(
    id: String,
    source: String,
    branch: String,
    pool: State<'_, SqlitePool>,
) -> Result<SkillDetail, String> {
    let skill_name = id.strip_prefix(&format!("{}/", source)).unwrap_or(&id);
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/SKILL.md",
        source, branch, skill_name
    );

    let client = crate::api::get_client();
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Failed to fetch skill: HTTP {} ({})", resp.status(), url));
    }

    let content = resp
        .text()
        .await
        .map_err(|e| format!("Read error: {}", e))?;

    let (name, description, instructions) = parse_skill_md(&content);

    let installed = list_installed_skill_ids(pool.inner()).await?.contains(&id);
    let install_path = if installed {
        Some(
            skills_dir()
                .join(&id)
                .to_string_lossy()
                .to_string(),
        )
    } else {
        None
    };

    Ok(SkillDetail {
        id,
        name,
        description,
        instructions,
        source,
        branch,
        installed,
        install_path,
    })
}

/// Tauri command to install a skill by downloading its SKILL.md and recording metadata in the database.
#[tauri::command]
pub async fn install_skill(
    id: String,
    source: String,
    branch: String,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    let skill_name = id.strip_prefix(&format!("{}/", source)).unwrap_or(&id);
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/SKILL.md",
        source, branch, skill_name
    );

    let client = crate::api::get_client();
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Failed to fetch skill: HTTP {}", resp.status()));
    }

    let content = resp
        .text()
        .await
        .map_err(|e| format!("Read error: {}", e))?;

    let skill_dir = skills_dir().join(&id);
    tokio::fs::create_dir_all(&skill_dir).await
        .map_err(|e| format!("Failed to create skill directory: {}", e))?;

    tokio::fs::write(skill_dir.join("SKILL.md"), &content).await
        .map_err(|e| format!("Failed to write skill file: {}", e))?;

    let (name, description, _) = parse_skill_md(&content);

    sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
    )
    .bind(format!("skill:{}:name", id))
    .bind(&name)
    .execute(pool.inner())
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
    )
    .bind(format!("skill:{}:desc", id))
    .bind(&description)
    .execute(pool.inner())
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
    )
    .bind(format!("skill:{}:source", id))
    .bind(&source)
    .execute(pool.inner())
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
    )
    .bind(format!("skill:{}:branch", id))
    .bind(&branch)
    .execute(pool.inner())
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    let version = get_latest_version(&source).await?;
    sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
    )
    .bind(format!("skill:{}:version", id))
    .bind(&version)
    .execute(pool.inner())
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
    )
    .bind(format!("skill:{}:installed", id))
    .bind("true")
    .execute(pool.inner())
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(())
}

/// Tauri command to uninstall a skill by removing its directory and database entries.
#[tauri::command]
pub async fn uninstall_skill(
    id: String,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    let skill_dir = skills_dir().join(&id);
    if skill_dir.exists() {
        tokio::fs::remove_dir_all(&skill_dir).await
            .map_err(|e| format!("Failed to remove skill directory: {}", e))?;
    }

    let keys = ["name", "desc", "source", "branch", "version", "installed"];
    for key in &keys {
        let full_key = format!("skill:{}:{}", id, key);
        sqlx::query("DELETE FROM settings WHERE key = ?")
            .bind(&full_key)
            .execute(pool.inner())
            .await
            .map_err(|e| format!("Database error: {}", e))?;
    }

    Ok(())
}

/// Tauri command to list all currently installed skills with their metadata and instructions.
#[tauri::command]
pub async fn list_installed_skills(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<SkillDetail>, String> {
    let ids = list_installed_skill_ids(pool.inner()).await?;
    let mut skills = Vec::new();

    for id in ids {
        let name = get_skill_setting(pool.inner(), &id, "name").await?;
        let description = get_skill_setting(pool.inner(), &id, "desc").await?;
        let source = get_skill_setting(pool.inner(), &id, "source").await?;

        let skill_path = skills_dir().join(&id).join("SKILL.md");
        let instructions = if skill_path.exists() {
            tokio::fs::read_to_string(&skill_path).await.unwrap_or_default()
        } else {
            String::new()
        };

        skills.push(SkillDetail {
            id: id.clone(),
            name,
            description,
            instructions,
            source,
            branch: String::new(),
            installed: true,
            install_path: Some(skills_dir().join(&id).to_string_lossy().to_string()),
        });
    }

    Ok(skills)
}

/// Tauri command to read the full SKILL.md instructions for an installed skill from disk.
#[tauri::command]
pub async fn get_skill_instructions(id: String) -> Result<String, String> {
    let skill_path = skills_dir().join(&id).join("SKILL.md");
    if !skill_path.exists() {
        return Err(format!("Skill '{}' is not installed", id));
    }

    tokio::fs::read_to_string(&skill_path).await
        .map_err(|e| format!("Failed to read skill: {}", e))
}

/// Tauri command to execute a skill action (read_file, write_file, list_dir, or run_command)
/// within the skill's workspace. Enforces path sandboxing and command allowlisting.
#[tauri::command]
pub async fn execute_skill_action(
    pool: tauri::State<'_, sqlx::SqlitePool>,
    id: String,
    action: SkillAction,
    workspace_path: Option<String>,
) -> Result<String, String> {
    let skill_dir = skills_dir().join(&id);
    if !skill_dir.exists() {
        return Err(format!("Skill '{}' is not installed", id));
    }

    let base_path = workspace_path.unwrap_or_else(|| {
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| String::from("."))
    });

    match action.action.as_str() {
        "read_file" => {
            let full = PathBuf::from(&base_path).join(&action.path);
            let canonical = full.canonicalize().map_err(|e| format!("Path error: {}", e))?;
            let base_canonical = PathBuf::from(&base_path)
                .canonicalize()
                .map_err(|e| format!("Base path error: {}", e))?;
            if !canonical.starts_with(&base_canonical) {
                return Err("Access denied: file is outside workspace".to_string());
            }
            tokio::fs::read_to_string(&canonical).await
                .map_err(|e| format!("Read error: {}", e))
        }
        "write_file" => {
            let content = action
                .content
                .ok_or_else(|| "Missing content for write_file".to_string())?;
            let full = PathBuf::from(&base_path).join(&action.path);
            let base_canonical = PathBuf::from(&base_path)
                .canonicalize()
                .map_err(|e| format!("Base path error: {}", e))?;
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
            if full_canonical != full && !full_canonical.starts_with(&base_canonical) {
                return Err("Access denied: file is outside workspace".to_string());
            }
            if let Some(parent) = full.parent() {
                if !parent.exists() {
                    tokio::fs::create_dir_all(parent).await
                        .map_err(|e| format!("Failed to create parent directories: {}", e))?;
                }
            }
            tokio::fs::write(&full, &content).await
                .map_err(|e| format!("Write error: {}", e))?;
            Ok(format!("Wrote to {}", action.path))
        }
        "list_dir" => {
            let full = PathBuf::from(&base_path).join(&action.path);
            let canonical = full.canonicalize().map_err(|e| format!("Path error: {}", e))?;
            let base_canonical = PathBuf::from(&base_path)
                .canonicalize()
                .map_err(|e| format!("Base path error: {}", e))?;
            if !canonical.starts_with(&base_canonical) {
                return Err("Access denied: directory is outside workspace".to_string());
            }
            let mut dir = tokio::fs::read_dir(&canonical).await
                .map_err(|e| format!("Read error: {}", e))?;
            let mut listing = Vec::new();
            while let Some(entry) = dir.next_entry().await
                .map_err(|e| format!("Read error: {}", e))? {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.file_type().await.map(|ft| ft.is_dir()).unwrap_or(false);
                listing.push(format!("{}{}", if is_dir { "[DIR]  " } else { "[FILE] " }, name));
            }
            listing.sort();
            Ok(listing.join("\n"))
        }
        "run_command" => {
            let command = action
                .command
                .ok_or_else(|| "Missing command".to_string())?;
            let cmd_name = command.split_whitespace().next().unwrap_or("");

            const METACHARS: &[char] = &[';', '|', '>', '<', '`'];
            if let Some(pos) = command.find(|c| METACHARS.contains(&c)) {
                return Err(format!(
                    "Shell metacharacter '{}' not allowed in commands",
                    command.chars().nth(pos).unwrap_or('?')
                ));
            }
            if command.contains("$(") {
                return Err("Command substitution $(...) is not allowed".into());
            }
            if command.contains("&&") {
                return Err("Shell operator && is not allowed".into());
            }
            if command.contains("||") {
                return Err("Shell operator || is not allowed".into());
            }

            let always_allowed = &[
                "npm", "npx", "pnpm", "yarn", "cargo", "pip", "pip3", "brew",
                "git", "node", "python3", "python", "ruby", "go", "deno",
                "make", "cmake", "gcc", "g++", "clang", "rustc",
                "ls", "cat", "head", "tail", "wc", "du", "df", "find", "grep",
                "which", "type", "file", "tree", "pwd", "realpath",
                "echo", "printf", "env", "uname", "whoami", "hostname",
                "tar", "gzip", "gunzip", "zip", "unzip",
            ];

            let file_cmds = &["mkdir", "touch", "cp", "mv", "rm", "ln", "chmod", "chown"];
            let network_cmds = &["curl", "wget", "nc", "telnet", "ssh"];
            let process_cmds = &["kill", "killall", "ps", "top", "systemctl"];

            let is_always = always_allowed.contains(&cmd_name);
            let is_file = file_cmds.contains(&cmd_name);
            let is_network = network_cmds.contains(&cmd_name);
            let is_process = process_cmds.contains(&cmd_name);

            if !is_always && !is_file && !is_network && !is_process {
                return Err(format!("Command '{}' not allowed", cmd_name));
            }

            if is_file {
                let allowed: bool = sqlx::query_scalar("SELECT value = 'true' FROM settings WHERE key = 'skill_allow_file_write'")
                    .fetch_optional(pool.inner()).await.map_err(|e| format!("DB error: {}", e))?.unwrap_or(false);
                if !allowed { return Err("File write commands blocked. Enable in Settings → Developer.".into()); }
            }
            if is_network {
                let allowed: bool = sqlx::query_scalar("SELECT value = 'true' FROM settings WHERE key = 'skill_allow_network'")
                    .fetch_optional(pool.inner()).await.map_err(|e| format!("DB error: {}", e))?.unwrap_or(false);
                if !allowed { return Err("Network commands blocked. Enable in Settings → Developer.".into()); }
            }
            if is_process {
                let allowed: bool = sqlx::query_scalar("SELECT value = 'true' FROM settings WHERE key = 'skill_allow_process'")
                    .fetch_optional(pool.inner()).await.map_err(|e| format!("DB error: {}", e))?.unwrap_or(false);
                if !allowed { return Err("Process commands blocked. Enable in Settings → Developer.".into()); }
            }

            let output = tokio::process::Command::new("sh")
                .arg("-c")
                .arg(&command)
                .current_dir(&base_path)
                .output()
                .await
                .map_err(|e| format!("Command error: {}", e))?;
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                Err(format!("Command failed: {}", stderr))
            }
        }
        _ => Err(format!("Unknown action: {}", action.action)),
    }
}

async fn list_installed_skill_ids(pool: &SqlitePool) -> Result<Vec<String>, String> {
    let keys = sqlx::query_scalar::<_, String>(
        "SELECT key FROM settings WHERE key LIKE 'skill:%:installed' AND value = 'true'",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(keys
        .into_iter()
        .filter_map(|k| {
            k.strip_prefix("skill:")
                .and_then(|s| s.strip_suffix(":installed"))
                .map(|s| s.to_string())
        })
        .collect())
}

async fn get_skill_setting(
    pool: &SqlitePool,
    id: &str,
    field: &str,
) -> Result<String, String> {
    let key = format!("skill:{}:{}", id, field);
    let row = sqlx::query_scalar::<_, String>("SELECT value FROM settings WHERE key = ?")
        .bind(&key)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Database error: {}", e))?;

    row.ok_or_else(|| format!("Skill setting not found: {}", key))
}

pub fn parse_skill_md(content: &str) -> (String, String, String) {
    let mut name = String::new();
    let mut description = String::new();
    let in_frontmatter = content.starts_with("---");

    let mut lines = content.lines();
    let mut in_fm = in_frontmatter;
    let mut first_line = true;

    for line in &mut lines {
        if in_fm {
            if line.trim() == "---" {
                if first_line {
                    first_line = false;
                    continue;
                }
                in_fm = false;
                continue;
            }
            first_line = false;
            if let Some(val) = line.strip_prefix("name:").or_else(|| line.strip_prefix("Name:")) {
                name = val.trim().trim_matches('"').to_string();
            }
            if let Some(val) = line
                .strip_prefix("description:")
                .or_else(|| line.strip_prefix("Description:"))
            {
                description = val.trim().trim_matches('"').to_string();
            }
        } else {
            break;
        }
    }

    if name.is_empty() {
        name = content
            .lines()
            .find(|l| l.starts_with("# "))
            .map(|l| l.trim_start_matches("# ").to_string())
            .unwrap_or_else(|| "Untitled Skill".to_string());
    }

    (name, description, content.to_string())
}

async fn get_latest_version(source: &str) -> Result<String, String> {
    let client = crate::api::get_client();
    match source {
        "anthropics/skills" => {
            let resp = client
                .get("https://api.github.com/repos/anthropics/skills/commits/main")
                .header("User-Agent", "clutch-app")
                .header("Accept", "application/vnd.github+json")
                .send()
                .await
                .map_err(|e| format!("Network error: {}", e))?;
            if !resp.status().is_success() {
                return Err(format!("GitHub API error: HTTP {}", resp.status()));
            }
            let json: serde_json::Value = resp
                .json()
                .await
                .map_err(|e| format!("Parse error: {}", e))?;
            json["sha"]
                .as_str()
                .map(|s| s[..7].to_string())
                .ok_or_else(|| "No SHA in response".into())
        }
        "obra/superpowers" => {
            let resp = client
                .get("https://api.github.com/repos/obra/superpowers/releases/latest")
                .header("User-Agent", "clutch-app")
                .header("Accept", "application/vnd.github+json")
                .send()
                .await
                .map_err(|e| format!("Network error: {}", e))?;
            if !resp.status().is_success() {
                return Err(format!("GitHub API error: HTTP {}", resp.status()));
            }
            let json: serde_json::Value = resp
                .json()
                .await
                .map_err(|e| format!("Parse error: {}", e))?;
            json["tag_name"]
                .as_str()
                .map(|s| s.to_string())
                .ok_or_else(|| "No tag_name in response".into())
        }
        other => Err(format!("Unknown source: {}", other)),
    }
}

#[tauri::command]
pub async fn check_skill_updates(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<SkillUpdateInfo>, String> {
    let ids = list_installed_skill_ids(pool.inner()).await?;
    let mut updates = Vec::new();
    let mut source_versions: HashMap<String, String> = HashMap::new();

    for id in ids {
        let name = get_skill_setting(pool.inner(), &id, "name").await?;
        let source = get_skill_setting(pool.inner(), &id, "source").await?;
        let stored_ver = get_skill_setting(pool.inner(), &id, "version").await.ok();

        let latest = if let Some(lv) = source_versions.get(&source) {
            lv.clone()
        } else {
            match get_latest_version(&source).await {
                Ok(v) => {
                    source_versions.insert(source.clone(), v.clone());
                    v
                }
                Err(e) => {
                    tracing::warn!("Failed to check updates for source {}: {}", source, e);
                    continue;
                }
            }
        };

        let has_update = stored_ver.as_ref().map_or(true, |v| v != &latest);

        updates.push(SkillUpdateInfo {
            id: id.clone(),
            name,
            has_update,
            current_version: stored_ver,
            latest_version: latest,
        });
    }

    Ok(updates)
}

#[tauri::command]
pub async fn update_skill(
    id: String,
    pool: State<'_, SqlitePool>,
) -> Result<(), String> {
    let source = get_skill_setting(pool.inner(), &id, "source").await?;
    if source.is_empty() {
        return Err("Skill not found or missing source".into());
    }
    let branch = get_skill_setting(pool.inner(), &id, "branch")
        .await
        .unwrap_or_else(|_| "main".to_string());
    let branch = if branch.is_empty() { "main".to_string() } else { branch };

    let skill_name = id.strip_prefix(&format!("{}/", source)).unwrap_or(&id);
    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/SKILL.md",
        source, branch, skill_name
    );

    let client = crate::api::get_client();
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Failed to fetch skill: HTTP {}", resp.status()));
    }

    let content = resp
        .text()
        .await
        .map_err(|e| format!("Read error: {}", e))?;

    let skill_dir = skills_dir().join(&id);
    tokio::fs::write(skill_dir.join("SKILL.md"), &content)
        .await
        .map_err(|e| format!("Failed to write skill file: {}", e))?;

    let (name, description, _) = parse_skill_md(&content);

    sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
    )
    .bind(format!("skill:{}:name", id))
    .bind(&name)
    .execute(pool.inner())
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
    )
    .bind(format!("skill:{}:desc", id))
    .bind(&description)
    .execute(pool.inner())
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    let version = get_latest_version(&source).await?;
    sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
    )
    .bind(format!("skill:{}:version", id))
    .bind(&version)
    .execute(pool.inner())
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    sqlx::query(
        "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?, ?, datetime('now'))",
    )
    .bind(format!("skill:{}:branch", id))
    .bind(&branch)
    .execute(pool.inner())
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    fn has_metachar(check: &str) -> bool {
        let m: &[char] = &[';', '|', '>', '<', '`'];
        check.find(|c| m.contains(&c)).is_some() || check.contains("$(") || check.contains("&&") || check.contains("||")
    }

    #[test]
    fn metachar_rejects_semicolon_injection() {
        assert!(has_metachar("ls; rm -rf ~"), "should reject semicolon");
    }

    #[test]
    fn metachar_rejects_pipe_injection() {
        assert!(has_metachar("echo x | sh"), "should reject pipe");
    }

    #[test]
    fn metachar_rejects_command_substitution() {
        assert!(has_metachar("cat $(whoami)"), "should reject $()");
    }

    #[test]
    fn metachar_rejects_backtick_substitution() {
        assert!(has_metachar("echo `id`"), "should reject backtick");
    }

    #[test]
    fn metachar_rejects_double_ampersand() {
        assert!(has_metachar("echo x && curl evil"), "should reject &&");
    }

    #[test]
    fn metachar_rejects_redirect() {
        assert!(has_metachar("echo > /etc/passwd"), "should reject >");
    }

    #[test]
    fn metachar_allows_env_var() {
        assert!(!has_metachar("echo $HOME"), "should allow bare $");
    }

    #[test]
    fn metachar_allows_simple_command() {
        assert!(!has_metachar("ls -la"), "should allow simple commands");
    }

    #[test]
    fn metachar_allows_git_status() {
        assert!(!has_metachar("git status"), "should allow git");
    }

    #[test]
    fn metachar_allows_npm_install() {
        assert!(!has_metachar("npm install"), "should allow npm");
    }

    #[test]
    fn metachar_rejects_double_or() {
        assert!(has_metachar("echo x || rm -rf ~"), "should reject ||");
    }
}
