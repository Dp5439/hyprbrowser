use iced::widget::*;
use iced::Element;

#[derive(Debug, Clone)]
pub struct Permission {
    pub name: String,
    pub description: String,
    pub granted: bool,
}

pub fn view(_browser: &crate::browser::BrowserState) -> Element<crate::Message> {
    let permissions = vec![
        Permission {
            name: "Camera".to_string(),
            description: "Allow websites to access your camera".to_string(),
            granted: false,
        },
        Permission {
            name: "Microphone".to_string(),
            description: "Allow websites to access your microphone".to_string(),
            granted: false,
        },
        Permission {
            name: "Location".to_string(),
            description: "Allow websites to access your location".to_string(),
            granted: false,
        },
        Permission {
            name: "Notifications".to_string(),
            description: "Allow websites to send notifications".to_string(),
            granted: false,
        },
    ];

    let mut perms_column = iced::widget::column![text("Permissions").size(16)].spacing(12).padding(16);

    for perm in permissions {
        let checkbox_row = row![
            checkbox(&perm.name, perm.granted)
                .on_toggle(|_| crate::Message::TogglePermissions),
            iced::widget::column![
                text(&perm.description).size(12),
            ]
            .spacing(4),
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center);

        perms_column = perms_column.push(checkbox_row);
    }

    container(
        scrollable(perms_column)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill),
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .padding(8)
    .into()
}
