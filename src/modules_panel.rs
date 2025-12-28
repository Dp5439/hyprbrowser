use iced::widget::*;
use iced::Element;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub version: String,
    pub enabled: bool,
    pub installed: bool,
}

pub fn view(_browser: &crate::browser::BrowserState) -> Element<crate::Message> {
    let modules = vec![
        Module {
            name: "Dark Mode".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
            installed: true,
        },
        Module {
            name: "Tab Groups".to_string(),
            version: "1.0.0".to_string(),
            enabled: false,
            installed: true,
        },
        Module {
            name: "Privacy Plus".to_string(),
            version: "1.0.0".to_string(),
            enabled: false,
            installed: false,
        },
    ];

    let search_input = text_input("Search modules...", "")
        .padding(8)
        .width(iced::Length::Fill)
        .on_input(|_| crate::Message::ToggleModules);

    let filter_pills = row![
        button(text("All")).padding(6),
        button(text("Installed")).padding(6),
        button(text("Enabled")).padding(6),
        button(text("Disabled")).padding(6),
    ]
    .spacing(8)
    .padding(8);

    let mut modules_list = iced::widget::column![search_input, filter_pills]
        .spacing(8)
        .padding(16);

    for module in modules {
        let toggle = checkbox("", module.enabled)
            .on_toggle(|_| crate::Message::ToggleModules);

        let install_btn = if module.installed {
            button(text("✓")).padding(6).on_press(crate::Message::ToggleModules)
        } else {
            button(text("⬇")).padding(6).on_press(crate::Message::ToggleModules)
        };

        let module_row = row![
            toggle,
            iced::widget::column![
                text(&module.name).size(12),
                text(format!("v{}", module.version)).size(10),
            ]
            .spacing(2)
            .width(iced::Length::Fill),
            install_btn,
            button(text("CFG")).padding(6).on_press(crate::Message::ToggleModules),
            button(text("X")).padding(6).on_press(crate::Message::ToggleModules),
        ]
        .spacing(8)
        .align_items(iced::Alignment::Center);

        modules_list = modules_list.push(module_row);
    }

    let upload_btn = button(text("UPLOAD MODULE"))
        .padding(8)
        .on_press(crate::Message::ToggleModules);

    modules_list = modules_list.push(upload_btn);

    container(
        scrollable(modules_list)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill),
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .padding(8)
    .into()
}
