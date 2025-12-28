use iced::widget::*;
use iced::Element;

pub fn view(browser: &crate::browser::BrowserState) -> Element<crate::Message> {
    let pinned_tabs = browser.tabs.get_pinned_tabs();

    let mut sidebar_col = self::column![
        button(text("HOME").size(12))
            .padding(12)
            .width(iced::Length::Fill)
            .on_press(crate::Message::NewTab),
        button(text("DOWNLOADS").size(12))
            .padding(12)
            .width(iced::Length::Fill)
            .on_press(crate::Message::ToggleDownloads),
        button(text("HISTORY").size(12))
            .padding(12)
            .width(iced::Length::Fill)
            .on_press(crate::Message::ToggleHistory),
        button(text("MODULES").size(12))
            .padding(12)
            .width(iced::Length::Fill)
            .on_press(crate::Message::ToggleModules),
        button(text("WORKFLOW").size(12))
            .padding(12)
            .width(iced::Length::Fill)
            .on_press(crate::Message::ToggleWorkflow),
        button(text("KEYS").size(12))
            .padding(12)
            .width(iced::Length::Fill)
            .on_press(crate::Message::ToggleKeybindings),
        button(text("PERMS").size(12))
            .padding(12)
            .width(iced::Length::Fill)
            .on_press(crate::Message::TogglePermissions),
        button(text("UPDATE").size(12))
            .padding(12)
            .width(iced::Length::Fill)
            .on_press(crate::Message::ToggleUpdater),
    ]
    .spacing(8)
    .padding(8);

    if !pinned_tabs.is_empty() {
        sidebar_col = sidebar_col.push(text("Pinned:").size(12));
        for (idx, tab) in pinned_tabs {
            sidebar_col = sidebar_col.push(
                button(text(&tab.title).size(10))
                    .padding(4)
                    .on_press(crate::Message::SelectTab(idx)),
            );
        }
    }

    container(
        scrollable(sidebar_col)
            .width(iced::Length::Shrink)
            .height(iced::Length::Fill),
    )
    .width(100)
    .height(iced::Length::Fill)
    .padding(8)
    .into()
}
