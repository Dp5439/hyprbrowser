use iced::widget::{button, container, row, scrollable, text, Column};
use iced::Element;

#[derive(Debug, Clone)]
pub struct Keybinding {
    pub name: String,
    pub keys: String,
    pub editable: bool,
}

pub fn view(_browser: &crate::browser::BrowserState) -> Element<crate::Message> {
    let keybindings = vec![
        Keybinding {
            name: "New Tab".to_string(),
            keys: "Shift+T".to_string(),
            editable: false,
        },
        Keybinding {
            name: "New Incognito Tab".to_string(),
            keys: "Shift+Ctrl+T".to_string(),
            editable: false,
        },
        Keybinding {
            name: "Duplicate Tab".to_string(),
            keys: "Shift+D".to_string(),
            editable: false,
        },
        Keybinding {
            name: "Close Other Tabs".to_string(),
            keys: "Shift+O".to_string(),
            editable: false,
        },
        Keybinding {
            name: "Toggle Multi-Panel".to_string(),
            keys: "Shift+P".to_string(),
            editable: false,
        },
        Keybinding {
            name: "Toggle Adblock".to_string(),
            keys: "Shift+B".to_string(),
            editable: false,
        },
        Keybinding {
            name: "Focus URL Bar".to_string(),
            keys: "Shift+U".to_string(),
            editable: false,
        },
        Keybinding {
            name: "Go Home".to_string(),
            keys: "Shift+H".to_string(),
            editable: false,
        },
        Keybinding {
            name: "Quick Search".to_string(),
            keys: "Shift+Tab".to_string(),
            editable: false,
        },
    ];

    let mut keybindings_col = Column::new()
        .push(text("Keybindings").size(16))
        .spacing(12)
        .padding(16);

    for kb in keybindings {
        let row = row![
            text(&kb.name).size(12),
            text(&kb.keys).size(12).width(iced::Length::Fill),
        ]
        .spacing(16)
        .align_items(iced::Alignment::Center);

        keybindings_col = keybindings_col.push(row);
    }

    let doc_btn = button(text("📖 Full Documentation"))
        .padding(8)
        .on_press(crate::Message::ToggleKeybindings);

    keybindings_col = keybindings_col.push(doc_btn);

    container(
        scrollable(keybindings_col)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill),
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .padding(8)
    .into()
}
