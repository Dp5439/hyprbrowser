use iced::widget::{button, column, container, row, text};
use iced::Element;
use tokio::fs;
use reqwest::Client;
use std::path::PathBuf;
use zip::ZipArchive;

pub struct Updater {
    pub checking: bool,
    pub new_version_available: bool,
    pub current_version: String,
    pub latest_version: String,
}

impl Updater {
    pub fn new() -> Self {
        Updater {
            checking: false,
            new_version_available: false,
            current_version: env!("CARGO_PKG_VERSION").to_string(),
            latest_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub async fn check_for_updates(&mut self) -> anyhow::Result<()> {
        self.checking = true;

        let client = Client::new();
        // Fetch latest release from GitHub
        let url = "https://api.github.com/repos/pro-grammer-SD/hyprbrowser/releases/latest";

        match client.get(url).header("User-Agent", "HyprBrowser").send().await {
            Ok(resp) => {
                if let Ok(text) = resp.text().await {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(tag) = json.get("tag_name").and_then(|v| v.as_str()) {
                            self.latest_version = tag.trim_start_matches('v').to_string();
                            // Compare versions properly
                            self.new_version_available = self.is_newer_version(tag.trim_start_matches('v'));
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to check for updates: {}", e);
            }
        }

        self.checking = false;
        Ok(())
    }

    fn is_newer_version(&self, remote_version: &str) -> bool {
        // Simple version comparison
        let current_parts: Vec<u32> = self.current_version
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        let remote_parts: Vec<u32> = remote_version
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();

        for i in 0..current_parts.len().max(remote_parts.len()) {
            let current = current_parts.get(i).unwrap_or(&0);
            let remote = remote_parts.get(i).unwrap_or(&0);
            
            if remote > current {
                return true;
            } else if remote < current {
                return false;
            }
        }
        false
    }

    #[cfg(target_os = "windows")]
    pub async fn download_and_update(&self) -> anyhow::Result<()> {
        log::info!("Downloading release for version {}", self.latest_version);

        let client = Client::new();
        let url = format!(
            "https://github.com/pro-grammer-SD/hyprbrowser/releases/download/v{}/hyprbrowser-windows.zip",
            self.latest_version
        );

        let response = client.get(&url).send().await?;
        let bytes = response.bytes().await?;

        // Extract to temporary location
        let temp_dir = PathBuf::from("./temp_update");
        fs::create_dir_all(&temp_dir).await?;

        // Save zip and extract
        let zip_path = temp_dir.join("release.zip");
        tokio::fs::write(&zip_path, bytes).await?;

        // Extract
        let file = std::fs::File::open(&zip_path)?;
        let mut archive = ZipArchive::new(file)?;
        archive.extract(&temp_dir)?;

        // Replace executable
        let dist_dir = PathBuf::from("./dist");
        fs::create_dir_all(&dist_dir).await?;

        let new_exe = temp_dir.join("hyprbrowser.exe");
        let target_exe = dist_dir.join("hyprbrowser.exe");
        if new_exe.exists() {
            tokio::fs::rename(new_exe, target_exe).await?;
        }

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir).await;

        log::info!("Update completed successfully!");
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub async fn download_and_update(&self) -> anyhow::Result<()> {
        anyhow::bail!("Auto-update is only supported on Windows")
    }
}

pub fn view(updater: &Updater) -> Element<crate::Message> {
    let current_version = &updater.current_version;
    let latest_version = &updater.latest_version;

    let version_info = row![
        column![
            text("Current Version:").size(12),
            text(current_version).size(14),
        ]
        .spacing(4),
        column![
            text("Latest Version:").size(12),
            text(latest_version).size(14),
        ]
        .spacing(4),
    ]
    .spacing(24)
    .padding(16);

    let check_btn = if updater.checking {
        button(text("Checking..."))
            .padding(8)
    } else {
        button(text("Check for Updates"))
            .padding(8)
            .on_press(crate::Message::CheckForUpdates)
    };

    let update_section = if updater.new_version_available {
        column![
            text(format!("New version {} available!", latest_version))
                .size(14)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.0, 0.8, 0.0))),
            button(text("DOWNLOAD UPDATE"))
                .padding(8)
                .on_press(crate::Message::DownloadUpdate),
        ]
        .spacing(8)
    } else {
        column![
            text("You're up to date!")
                .size(14)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.0, 0.8, 0.0))),
        ]
        .spacing(8)
    };

    container(
        column![
            text("Updater").size(16),
            version_info,
            check_btn,
            update_section,
            text("Your data and profile will be preserved during the update.").size(10),
        ]
        .spacing(12)
        .padding(16),
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .padding(8)
    .into()
}
