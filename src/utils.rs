use std::path::PathBuf;

pub fn get_dist_dir() -> PathBuf {
    PathBuf::from("./dist")
}

pub fn get_data_dir() -> anyhow::Result<PathBuf> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot find data directory"))?;

    Ok(data_dir.join("hyprbrowser").join("data"))
}

pub fn get_profile_dir() -> anyhow::Result<PathBuf> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot find data directory"))?;

    Ok(data_dir.join("hyprbrowser").join("profile"))
}

pub fn get_assets_dir() -> PathBuf {
    PathBuf::from("./assets")
}

pub fn ensure_directories() -> anyhow::Result<()> {
    std::fs::create_dir_all(get_dist_dir())?;
    std::fs::create_dir_all(get_data_dir()?)?;
    std::fs::create_dir_all(get_profile_dir()?)?;
    std::fs::create_dir_all(get_assets_dir())?;

    Ok(())
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len])
    } else {
        s.to_string()
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;

    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_idx])
}

pub fn get_timestamp() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Check if a URL is valid
pub fn is_valid_url(s: &str) -> bool {
    url::Url::parse(s).is_ok()
}

/// Convert a string to a URL (add https:// if missing)
pub fn to_url(s: &str) -> String {
    if s.starts_with("http://") || s.starts_with("https://") {
        s.to_string()
    } else if s.contains('.') {
        format!("https://{}", s)
    } else {
        google_search_url(s)
    }
}

pub fn google_search_url(query: &str) -> String {
    let encoded = urlencoding::encode(query);
    format!("https://www.google.com/search?q={}", encoded)
}
