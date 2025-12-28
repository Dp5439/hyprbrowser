use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::theme::ThemeMode;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub window_size: Option<(u32, u32)>,
    pub theme_mode: ThemeMode,
    pub open_tabs: Vec<String>,
    pub current_tab_index: usize,
    pub adblock_enabled: bool,
    pub vpn_enabled: bool,
}

impl State {
    pub fn new() -> Self {
        State {
            window_size: Some((1280, 720)),
            theme_mode: ThemeMode::System,
            open_tabs: vec!["https://www.google.com".to_string()],
            current_tab_index: 0,
            adblock_enabled: true,
            vpn_enabled: false,
        }
    }

    pub fn load() -> anyhow::Result<Self> {
        let state_dir = Self::get_state_dir()?;
        let state_file = state_dir.join("state.json");

        if state_file.exists() {
            let contents = std::fs::read_to_string(&state_file)?;
            let state = serde_json::from_str(&contents)?;
            Ok(state)
        } else {
            Ok(State::new())
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let state_dir = Self::get_state_dir()?;
        std::fs::create_dir_all(&state_dir)?;

        let state_file = state_dir.join("state.json");
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(&state_file, contents)?;

        Ok(())
    }

    pub fn save_browser_state(browser: &crate::browser::BrowserState) -> anyhow::Result<()> {
        let state_dir = Self::get_state_dir()?;
        std::fs::create_dir_all(&state_dir)?;

        let mut state = State::load().unwrap_or_else(|_| State::new());
        state.theme_mode = browser.theme_mode;
        state.adblock_enabled = browser.adblock_enabled;
        state.vpn_enabled = browser.vpn_enabled;

        state.save()?;
        Ok(())
    }

    pub fn save_theme(theme_mode: &ThemeMode) -> anyhow::Result<()> {
        let state_dir = Self::get_state_dir()?;
        std::fs::create_dir_all(&state_dir)?;

        let mut state = State::load().unwrap_or_else(|_| State::new());
        state.theme_mode = *theme_mode;

        state.save()?;
        Ok(())
    }

    fn get_state_dir() -> anyhow::Result<PathBuf> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot find data directory"))?;

        Ok(data_dir.join("hyprbrowser"))
    }
}
