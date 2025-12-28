use iced::widget::*;
use iced::Element;

#[derive(Debug, Clone)]
pub struct Tab {
    pub url: String,
    pub title: String,
    pub favicon: Option<String>,
    pub pinned: bool,
    pub incognito: bool,
    pub history: Vec<String>,
    pub history_pos: usize,
}

impl Tab {
    pub fn new(url: String, incognito: bool) -> Self {
        Tab {
            title: "Loading...".to_string(),
            favicon: None,
            pinned: false,
            incognito,
            history: vec![url.clone()],
            history_pos: 0,
            url,
        }
    }
}

pub struct TabManager {
    pub tabs: Vec<Tab>,
}

impl TabManager {
    pub fn new() -> Self {
        TabManager { tabs: Vec::new() }
    }

    pub fn new_tab(&mut self, url: &str, incognito: bool) {
        self.tabs.push(Tab::new(url.to_string(), incognito));
    }

    pub fn get_current(&self, index: usize) -> Option<&Tab> {
        self.tabs.get(index)
    }

    pub fn get_pinned_tabs(&self) -> Vec<(usize, &Tab)> {
        self.tabs
            .iter()
            .enumerate()
            .filter(|(_, tab)| tab.pinned)
            .collect()
    }
}

pub fn view_tabs(browser: &crate::browser::BrowserState) -> Element<crate::Message> {
    let mut tab_buttons = row![];

    for (idx, tab) in browser.tabs.tabs.iter().enumerate() {
        let pin_button = if tab.pinned {
            button(text("PIN").size(10)).on_press(crate::Message::UnpinTab(idx))
        } else {
            button(text("PIN").size(10)).on_press(crate::Message::PinTab(idx))
        };
        
        let tab_button = button(
            row![
                text(&tab.title).size(12),
                pin_button.padding(2),
                button(text("X").size(10))
                    .on_press(crate::Message::CloseTab(idx))
                    .padding(2),
            ]
            .spacing(5)
            .padding(5),
        )
        .on_press(crate::Message::SelectTab(idx))
        .padding(8);

        tab_buttons = tab_buttons.push(tab_button);
    }

    container(tab_buttons.spacing(5).padding(10))
        .width(iced::Length::Fill)
        .height(iced::Length::Shrink)
        .into()
}

pub fn view_address_bar(browser: &crate::browser::BrowserState) -> Element<crate::Message> {
    let address_input = text_input(
        "https://example.com",
        &browser.address_bar_input,
    )
    .on_input(|s| crate::Message::BrowserMessage(crate::browser::BrowserMessage::AddressBarInput(s)))
    .padding(10)
    .size(14)
    .width(iced::Length::Fill);

    let button_row = row![
        button(text("← Back")).padding(8),
        button(text("⟳ Reload")).padding(8),
        button(text("⋮ Menu")).padding(8),
    ]
    .spacing(5);

    container(
        row![address_input, button_row]
            .spacing(8)
            .padding(10)
            .align_items(iced::Alignment::Center),
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Shrink)
    .into()
}
