use iced::widget::text;
use iced::Element;

pub const ICON_HOME: &str = "HOME";
pub const ICON_DOWNLOADS: &str = "DOWNLOADS";
pub const ICON_HISTORY: &str = "HISTORY";
pub const ICON_MODULES: &str = "MODULES";
pub const ICON_WORKFLOW: &str = "WORKFLOW";
pub const ICON_KEYBINDINGS: &str = "KEYS";
pub const ICON_PERMISSIONS: &str = "PERMS";
pub const ICON_UPDATER: &str = "UPDATE";
pub const ICON_ADBLOCK: &str = "BLOCK";
pub const ICON_VPN: &str = "VPN";
pub const ICON_DEVTOOLS: &str = "DEBUG";
pub const ICON_SETTINGS: &str = "SETTINGS";
pub const ICON_SEARCH: &str = "SEARCH";
pub const ICON_PIN: &str = "PIN";
pub const ICON_CLOSE: &str = "X";

pub fn render_icon(name: &str) -> Element<'static, crate::Message> {
    match name {
        "home" => text(ICON_HOME).size(14).into(),
        "downloads" => text(ICON_DOWNLOADS).size(14).into(),
        "history" => text(ICON_HISTORY).size(14).into(),
        "modules" => text(ICON_MODULES).size(14).into(),
        "workflow" => text(ICON_WORKFLOW).size(14).into(),
        "keybindings" => text(ICON_KEYBINDINGS).size(14).into(),
        "permissions" => text(ICON_PERMISSIONS).size(14).into(),
        "updater" => text(ICON_UPDATER).size(14).into(),
        "adblock" => text(ICON_ADBLOCK).size(14).into(),
        "vpn" => text(ICON_VPN).size(14).into(),
        "devtools" => text(ICON_DEVTOOLS).size(14).into(),
        "search" => text(ICON_SEARCH).size(14).into(),
        "pin" => text(ICON_PIN).size(12).into(),
        "close" => text(ICON_CLOSE).size(12).into(),
        _ => text("?").size(14).into(),
    }
}

/// Get placeholder color for icon backgrounds
pub fn get_icon_color(index: usize) -> iced::Color {
    let colors = [
        iced::Color::from_rgb(0.2, 0.4, 0.8),
        iced::Color::from_rgb(0.8, 0.2, 0.4),
        iced::Color::from_rgb(0.2, 0.8, 0.4),
        iced::Color::from_rgb(0.8, 0.8, 0.2),
    ];
    colors[index % colors.len()]
}
