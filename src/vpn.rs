use iced::widget::*;
use iced::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VpnStatus {
    Connected,
    Disconnected,
    Connecting,
    Error,
}

pub struct VpnManager {
    pub status: VpnStatus,
    pub server: String,
    pub ip: String,
}

impl VpnManager {
    pub fn new() -> Self {
        VpnManager {
            status: VpnStatus::Disconnected,
            server: "None".to_string(),
            ip: "0.0.0.0".to_string(),
        }
    }

    pub fn toggle(&mut self) {
        self.status = match self.status {
            VpnStatus::Connected => VpnStatus::Disconnected,
            VpnStatus::Disconnected => VpnStatus::Connecting,
            VpnStatus::Connecting => VpnStatus::Connected,
            VpnStatus::Error => VpnStatus::Disconnected,
        };
    }

    pub fn connect(&mut self, server: String) {
        self.status = VpnStatus::Connecting;
        self.server = server;
        // In production, implement actual VPN connection logic
    }

    pub fn disconnect(&mut self) {
        self.status = VpnStatus::Disconnected;
        self.ip = "0.0.0.0".to_string();
    }
}

pub fn view() -> Element<'static, crate::Message> {
    let status_text = "VPN: Disconnected";
    let status_button = button(text("Toggle VPN"))
        .padding(8)
        .on_press(crate::Message::ToggleVpn);

    container(
        iced::widget::column![
            text(status_text).size(14),
            status_button,
            text("Select a VPN Server:").size(12),
            pick_list(
                vec!["US - California", "EU - Netherlands", "Asia - Singapore"],
                Some("US - California"),
                |_| crate::Message::ToggleVpn,
            )
            .padding(8),
        ]
        .spacing(8)
        .padding(12),
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Shrink)
    .into()
}
