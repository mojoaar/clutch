use futures_util::future::join_all;
use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tokio::time::Duration;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchOptions {
    pub timeout_secs: Option<u64>,
    pub max_size_bytes: Option<u64>,
    pub mode: Option<String>, // "raw", "markdown", "info"
    pub headers: Option<Vec<(String, String)>>,
    pub follow_redirects: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchResult {
    pub url: String,
    pub content: String,
    pub status_code: u16,
    pub content_type: Option<String>,
    pub content_length: Option<u64>,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BatchFetchResult {
    pub results: Vec<Option<FetchResult>>,
    pub errors: Vec<(String, String)>,
}

fn is_private_ip(ip: std::net::SocketAddr) -> bool {
    match ip.ip() {
        IpAddr::V4(v4) => {
            v4.is_loopback()
                || v4.is_private()
                || v4.is_link_local()
                || (v4.octets()[0] == 100 && v4.octets()[1] >= 64 && v4.octets()[1] <= 127) // 100.64.0.0/10
                || (v4.octets()[0] == 169 && v4.octets()[1] == 254) // 169.254.0.0/16
                || v4.octets() == [0, 0, 0, 0]
        }
        IpAddr::V6(v6) => {
            v6.is_loopback()
                || v6.is_unspecified()
                || v6.is_unique_local() // fc00::/7
        }
    }
}

fn validate_url(url_str: &str) -> Result<Url, String> {
    let url = Url::parse(url_str).map_err(|e| format!("Invalid URL: {}", e))?;

    let scheme = url.scheme();
    if scheme != "http" && scheme != "https" {
        return Err(format!("Unsupported scheme: {}", scheme));
    }

    if let Some(host) = url.host_str() {
        let lower = host.to_lowercase();
        if lower == "localhost" || lower.ends_with(".local") {
            return Err("Local hostname blocked".into());
        }
    }

    Ok(url)
}

async fn resolve_url(url: &Url) -> Result<std::net::SocketAddr, String> {
    let host = url
        .host_str()
        .ok_or_else(|| "No host in URL".to_string())?;
    let port = url.port().unwrap_or_else(|| {
        if url.scheme() == "https" {
            443
        } else {
            80
        }
    });

    let addrs: Vec<_> = tokio::net::lookup_host(format!("{}:{}", host, port))
        .await
        .map_err(|e| format!("DNS lookup failed: {}", e))?
        .collect();

    if addrs.is_empty() {
        return Err("No addresses resolved".into());
    }

    Ok(addrs[0])
}

fn extract_webpage_info(html_str: &str, _url: &str) -> (Option<String>, Option<String>) {
    let document = Html::parse_document(html_str);
    let title_sel = Selector::parse("title").unwrap();
    let meta_sel = Selector::parse("meta").unwrap();

    let title = document
        .select(&title_sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string());

    let description = document.select(&meta_sel).find_map(|el| {
        let name = el.value().attr("name").unwrap_or("").to_lowercase();
        let property = el.value().attr("property").unwrap_or("").to_lowercase();
        if name == "description"
            || property == "og:description"
        {
            el.value().attr("content").map(|c| c.to_string())
        } else {
            None
        }
    });

    (title, description)
}

/// Tauri command to fetch a URL with configurable options for timeout, size limit, mode (raw/markdown/info), and redirects.
/// Blocks requests to private/internal IP addresses for security.
#[tauri::command]
pub async fn fetch_url(
    url: String,
    options: Option<FetchOptions>,
) -> Result<FetchResult, String> {
    let opts = options.unwrap_or(FetchOptions {
        timeout_secs: None,
        max_size_bytes: None,
        mode: None,
        headers: None,
        follow_redirects: None,
    });

    let parsed = validate_url(&url)?;
    let addr = resolve_url(&parsed).await?;

    if is_private_ip(addr) {
        return Err("Private/internal IP addresses are blocked".into());
    }

    let timeout = Duration::from_secs(opts.timeout_secs.unwrap_or(30));
    let max_size = opts.max_size_bytes.unwrap_or(10 * 1024 * 1024); // 10MB default
    let max_redirects = opts.follow_redirects.unwrap_or(10);
    let mode = opts.mode.as_deref().unwrap_or("raw");

    let mut headers = HeaderMap::new();
    if let Some(hdrs) = &opts.headers {
        for (k, v) in hdrs {
            if let (Ok(name), Ok(value)) = (
                HeaderName::from_bytes(k.as_bytes()),
                HeaderValue::from_str(v),
            ) {
                headers.insert(name, value);
            }
        }
    }

    let client = reqwest::Client::builder()
        .timeout(timeout)
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(headers.clone())
        .user_agent("Clutch/0.2")
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let mut current_url = parsed.clone();
    let mut remaining = max_redirects;
    let (status_code, content_type, content_length, raw_bytes) = loop {
        let resp = client
            .get(current_url.as_str())
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if resp.status().is_redirection() && remaining > 0 {
            let location = resp
                .headers()
                .get(reqwest::header::LOCATION)
                .ok_or_else(|| "Redirect response missing Location header".to_string())?
                .to_str()
                .map_err(|e| format!("Invalid Location header: {}", e))?;

            let redirect_url = Url::options()
                .base_url(Some(&current_url))
                .parse(location)
                .map_err(|e| format!("Invalid redirect URL: {}", e))?;

            let validated = validate_url(redirect_url.as_str())?;
            let redirect_addr = resolve_url(&validated).await?;
            if is_private_ip(redirect_addr) {
                return Err("Redirect to private/internal IP address is blocked".into());
            }

            current_url = validated;
            remaining -= 1;
            continue;
        }

        let status = resp.status().as_u16();
        let ct = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());
        let cl = resp.content_length();

        if let Some(len) = cl {
            if len > max_size as u64 {
                return Err(format!(
                    "Response Content-Length {} exceeds limit {}",
                    len, max_size
                ));
            }
        }

        let mut raw_bytes = Vec::new();
        let mut body_stream = resp.bytes_stream();
        while let Some(chunk_result) = body_stream.next().await {
            let chunk = chunk_result.map_err(|e| format!("Failed to read response body: {}", e))?;
            raw_bytes.extend_from_slice(&chunk);
            if raw_bytes.len() > max_size as usize {
                return Err(format!("Response size exceeds limit {}", max_size));
            }
        }

        break (status, ct, cl, raw_bytes);
    };

    let raw_text = String::from_utf8_lossy(&raw_bytes).to_string();
    let is_html = content_type
        .as_ref()
        .map(|ct| ct.contains("html") || ct.contains("xml"))
        .unwrap_or(false);

    let (content, title, description) = match mode {
        "markdown" if is_html => {
            let (t, d) = extract_webpage_info(&raw_text, current_url.as_str());
            let md = html2md::parse_html(&raw_text);
            (md, t, d)
        }
        "info" => {
            let (t, d) = extract_webpage_info(&raw_text, current_url.as_str());
            (String::new(), t, d)
        }
        _ => (raw_text, None, None),
    };

    Ok(FetchResult {
        url: current_url.to_string(),
        content,
        status_code,
        content_type,
        content_length,
        title,
        description,
    })
}

/// Tauri command to fetch multiple URLs concurrently with the same options applied to each request.
/// Returns both successful results and per-URL error messages.
#[tauri::command]
pub async fn batch_fetch(
    urls: Vec<String>,
    options: Option<FetchOptions>,
) -> BatchFetchResult {
    let futures: Vec<_> = urls
        .iter()
        .map(|url| {
            let url = url.clone();
            let options = options.clone();
            async move {
                match fetch_url(url.clone(), options.clone()).await {
                    Ok(result) => Ok(result),
                    Err(e) => {
                        Err((url.clone(), e))
                    }
                }
            }
        })
        .collect();

    let outcomes = join_all(futures).await;

    let mut results: Vec<Option<FetchResult>> = Vec::with_capacity(outcomes.len());
    let mut errors: Vec<(String, String)> = Vec::new();

    for outcome in outcomes {
        match outcome {
            Ok(result) => results.push(Some(result)),
            Err(err) => {
                results.push(None);
                errors.push(err);
            }
        }
    }

    BatchFetchResult { results, errors }
}

/// Tauri command to fetch the README.md from a GitHub repository, trying main, master, and develop branches.
#[tauri::command]
pub async fn fetch_github_readme(repo_url: String) -> Result<FetchResult, String> {
    let parsed = Url::parse(&repo_url).map_err(|e| format!("Invalid URL: {}", e))?;

    if parsed.host_str() != Some("github.com") {
        return Err("Only github.com URLs are supported".into());
    }

    let path_segments: Vec<&str> = parsed
        .path_segments()
        .map(|s| s.collect())
        .unwrap_or_default();

    if path_segments.len() < 2 {
        return Err("Invalid GitHub repo URL".into());
    }

    let owner = path_segments[0];
    let repo = path_segments[1].trim_end_matches(".git");

    let branches = ["main", "master", "develop"];

    for branch in &branches {
        let url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}/README.md",
            owner, repo, branch
        );

        let opts = FetchOptions {
            timeout_secs: Some(15),
            max_size_bytes: Some(2 * 1024 * 1024),
            mode: Some("raw".into()),
            headers: None,
            follow_redirects: Some(3),
        };

        match fetch_url(url, Some(opts)).await {
            Ok(result) if result.status_code == 200 => return Ok(result),
            _ => continue,
        }
    }

    Err("README.md not found on main, master, or develop branches".into())
}

/// Tauri command to fetch metadata (title, description) from a webpage without downloading its full content.
#[tauri::command]
pub async fn fetch_webpage_info(url: String) -> Result<serde_json::Value, String> {
    let opts = FetchOptions {
        timeout_secs: Some(15),
        max_size_bytes: Some(2 * 1024 * 1024),
        mode: Some("info".into()),
        headers: None,
        follow_redirects: Some(5),
    };

    let result = fetch_url(url, Some(opts)).await?;

    Ok(serde_json::json!({
        "url": result.url,
        "title": result.title,
        "description": result.description,
        "status_code": result.status_code,
        "content_type": result.content_type,
        "content_length": result.content_length,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_private_ip_blocks_loopback_v4() {
        let addr: std::net::SocketAddr = "127.0.0.1:80".parse().unwrap();
        assert!(is_private_ip(addr), "should block 127.0.0.1");
    }

    #[test]
    fn is_private_ip_blocks_private_10() {
        let addr: std::net::SocketAddr = "10.0.0.1:443".parse().unwrap();
        assert!(is_private_ip(addr), "should block 10.0.0.1");
    }

    #[test]
    fn is_private_ip_blocks_private_192_168() {
        let addr: std::net::SocketAddr = "192.168.1.1:8080".parse().unwrap();
        assert!(is_private_ip(addr), "should block 192.168.1.1");
    }

    #[test]
    fn is_private_ip_blocks_link_local() {
        let addr: std::net::SocketAddr = "169.254.169.254:80".parse().unwrap();
        assert!(is_private_ip(addr), "should block 169.254.169.254");
    }

    #[test]
    fn is_private_ip_blocks_cgnat() {
        let addr: std::net::SocketAddr = "100.64.0.1:80".parse().unwrap();
        assert!(is_private_ip(addr), "should block 100.64.0.1");
    }

    #[test]
    fn is_private_ip_blocks_loopback_v6() {
        let addr: std::net::SocketAddr = "[::1]:80".parse().unwrap();
        assert!(is_private_ip(addr), "should block ::1");
    }

    #[test]
    fn is_private_ip_blocks_ula_v6() {
        let addr: std::net::SocketAddr = "[fc00::1]:80".parse().unwrap();
        assert!(is_private_ip(addr), "should block fc00::1");
    }

    #[test]
    fn is_private_ip_allows_public_ip() {
        let addr: std::net::SocketAddr = "8.8.8.8:443".parse().unwrap();
        assert!(!is_private_ip(addr), "should allow 8.8.8.8");
    }

    #[test]
    fn validate_url_accepts_https() {
        assert!(validate_url("https://example.com").is_ok());
    }

    #[test]
    fn validate_url_rejects_localhost() {
        assert!(validate_url("http://localhost").is_err());
    }

    #[test]
    fn validate_url_rejects_file_scheme() {
        assert!(validate_url("file:///etc/passwd").is_err());
    }

    #[test]
    fn validate_url_rejects_ftp_scheme() {
        assert!(validate_url("ftp://example.com").is_err());
    }
}
