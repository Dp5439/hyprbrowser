use iced::widget::*;
use iced::Element;

#[derive(Debug, Clone)]
pub struct Download {
    pub url: String,
    pub filename: String,
    pub progress: f32, // 0.0 to 1.0
    pub status: DownloadStatus,
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Paused,
    Completed,
    Failed,
}

pub struct DownloadManager {
    pub downloads: Vec<Download>,
    pub parallel_enabled: bool,
}

impl DownloadManager {
    pub fn new() -> Self {
        DownloadManager {
            downloads: Vec::new(),
            parallel_enabled: true,
        }
    }

    pub fn add_download(&mut self, url: String, filename: String) {
        self.downloads.push(Download {
            url,
            filename,
            progress: 0.0,
            status: DownloadStatus::Pending,
            size: 0,
        });
    }

    pub fn pause_download(&mut self, idx: usize) {
        if let Some(dl) = self.downloads.get_mut(idx) {
            dl.status = DownloadStatus::Paused;
        }
    }

    pub fn resume_download(&mut self, idx: usize) {
        if let Some(dl) = self.downloads.get_mut(idx) {
            dl.status = DownloadStatus::Downloading;
        }
    }

    pub fn cancel_download(&mut self, idx: usize) {
        if let Some(dl) = self.downloads.get_mut(idx) {
            dl.status = DownloadStatus::Failed;
        }
    }

    pub fn toggle_parallel(&mut self) {
        self.parallel_enabled = !self.parallel_enabled;
    }
}

pub fn view(browser: &crate::browser::BrowserState) -> Element<crate::Message> {
    let mut downloads_col = iced::widget::column![text("Downloads").size(16)].spacing(12).padding(16);

    let empty_msg = text("No downloads yet").size(12);

    downloads_col = downloads_col.push(empty_msg);

    let parallel_toggle = row![
        text("Parallel Downloads:").size(12),
        checkbox("", browser.parallel_downloads)
            .on_toggle(|_| crate::Message::ToggleDownloads),
    ]
    .spacing(8)
    .align_items(iced::Alignment::Center);

    downloads_col = downloads_col.push(parallel_toggle);

    container(
        scrollable(downloads_col)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill),
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .padding(8)
    .into()
}
