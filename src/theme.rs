use iced::Theme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

impl ThemeMode {
    pub fn to_string(&self) -> String {
        match self {
            ThemeMode::Light => "Light".to_string(),
            ThemeMode::Dark => "Dark".to_string(),
            ThemeMode::System => "System".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "Light" => ThemeMode::Light,
            "Dark" => ThemeMode::Dark,
            _ => ThemeMode::System,
        }
    }
}

pub fn get_current_theme() -> Theme {
    if cfg!(target_os = "windows") || cfg!(target_os = "linux") {
        Theme::Dark
    } else {
        Theme::Light
    }
}
