use iced::widget::{button, container, row, scrollable, text, text_input};
use iced::{widget::column, Element};
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub timestamp: DateTime<Local>,
}

pub struct HistoryManager {
    pub entries: Vec<HistoryEntry>,
}

impl HistoryManager {
    pub fn new() -> Self {
        HistoryManager {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, url: String, title: String) {
        self.entries.push(HistoryEntry {
            url,
            title,
            timestamp: Local::now(),
        });
    }

    pub fn clear_history(&mut self) {
        self.entries.clear();
    }

    pub fn search(&self, query: &str) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.url.contains(query) || e.title.contains(query))
            .collect()
    }
}

pub fn view(_browser: &crate::browser::BrowserState) -> Element<crate::Message> {
    let mut history_col = column![
        row![
            text("HISTORY").size(16),
            button(text("CLEAR"))
                .padding(8)
                .on_press(crate::Message::ToggleHistory),
        ]
        .spacing(12)
        .align_items(iced::Alignment::Center)
    ]
    .spacing(12)
    .padding(16);

    let search_input = text_input(
        "Search history...",
        ""
    )
    .padding(8);

    history_col = history_col.push(search_input);

    let empty_msg = text("No history yet").size(12);
    history_col = history_col.push(empty_msg);

    container(
        scrollable(history_col)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill),
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .padding(8)
    .into()
}
