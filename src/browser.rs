use crate::tabs::{Tab, TabManager}; 
use crate::theme::ThemeMode;
use crate::gpu_detect::{GpuInfo, GpuSettings};
use crate::updater_panel::Updater;  // Add this import
use iced::widget::{column, row, container, text};
use iced::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    None,
    Downloads,
    History,
    Modules,
    Workflow,
    Keybindings,
    Permissions,
    Updater,
}

#[derive(Debug, Clone)]
pub enum BrowserMessage {
    NavigateTo(String),
    AddressBarInput(String),
}

pub struct BrowserState {
    pub tabs: TabManager,
    pub current_tab: usize,
    pub address_bar_input: String,
    pub quick_search_input: String,
    pub quick_search_visible: bool,
    pub active_panel: Panel,
    pub multi_panel_enabled: bool,
    pub adblock_enabled: bool,
    pub vpn_enabled: bool,
    pub theme_mode: ThemeMode,
    pub snow_active: bool,
    pub snow_end_time: std::time::Instant,
    pub incognito_mode: bool,
    pub parallel_downloads: bool,
    pub gpu_info: GpuInfo,
    pub gpu_settings: GpuSettings,
    pub updater: Updater,  // Add this field
}

impl BrowserState {
    pub fn new(gpu_info: GpuInfo) -> Self {
        let mut tabs = TabManager::new();
        tabs.new_tab("https://www.google.com", false);

        let gpu_settings = GpuSettings::for_gpu(&gpu_info);
        
        log::info!("GPU Configuration: {}", gpu_info.to_string());
        log::info!(
            "Effect Intensity: {:.0}%, Particles: {}",
            gpu_settings.effect_intensity * 100.0,
            gpu_settings.particle_count
        );

        BrowserState {
            tabs,
            current_tab: 0,
            address_bar_input: String::new(),
            quick_search_input: String::new(),
            quick_search_visible: false,
            active_panel: Panel::None,
            multi_panel_enabled: false,
            adblock_enabled: true,
            vpn_enabled: false,
            theme_mode: ThemeMode::System,
            snow_active: false,
            snow_end_time: std::time::Instant::now(),
            incognito_mode: false,
            parallel_downloads: true,
            gpu_info,
            gpu_settings,
            updater: Updater::new(),  // Initialize the updater
        }
    }

    pub fn new_tab(&mut self) {
        self.tabs.new_tab("https://www.google.com", false);
        self.current_tab = self.tabs.tabs.len() - 1;
    }

    pub fn new_incognito_tab(&mut self) {
        self.tabs.new_tab("https://www.google.com", true);
        self.current_tab = self.tabs.tabs.len() - 1;
        self.incognito_mode = true;
    }

    pub fn close_tab(&mut self, idx: usize) {
        if self.tabs.tabs.len() > 1 {
            self.tabs.tabs.remove(idx);
            if self.current_tab >= self.tabs.tabs.len() {
                self.current_tab = self.tabs.tabs.len() - 1;
            }
        }
    }

    pub fn select_tab(&mut self, idx: usize) {
        if idx < self.tabs.tabs.len() {
            self.current_tab = idx;
        }
    }

    pub fn duplicate_tab(&mut self) {
        if let Some(tab) = self.tabs.tabs.get(self.current_tab) {
            let new_tab = Tab {
                url: tab.url.clone(),
                title: tab.title.clone(),
                favicon: tab.favicon.clone(),
                pinned: false,
                incognito: tab.incognito,
                history: vec![],
                history_pos: 0,
            };
            self.tabs.tabs.push(new_tab);
            self.current_tab = self.tabs.tabs.len() - 1;
        }
    }

    pub fn close_other_tabs(&mut self) {
        let current_tab = self.tabs.tabs.remove(self.current_tab);
        self.tabs.tabs.clear();
        self.tabs.tabs.push(current_tab);
        self.current_tab = 0;
    }

    pub fn pin_tab(&mut self, idx: usize) {
        if let Some(tab) = self.tabs.tabs.get_mut(idx) {
            tab.pinned = true;
        }
    }

    pub fn unpin_tab(&mut self, idx: usize) {
        if let Some(tab) = self.tabs.tabs.get_mut(idx) {
            tab.pinned = false;
        }
    }

    pub fn toggle_multi_panel(&mut self) {
        self.multi_panel_enabled = !self.multi_panel_enabled;
    }

    pub fn toggle_quick_search(&mut self) {
        self.quick_search_visible = !self.quick_search_visible;
    }

    pub fn execute_quick_search(&mut self) {
        if self.quick_search_input.is_empty() {
            return;
        }

        // Check for calculation
        if let Ok(result) = evalexpr::eval(&self.quick_search_input) {
            self.address_bar_input = format!("= {}", result);
        } else {
            // Google search
            let query = urlencoding::encode(&self.quick_search_input);
            self.address_bar_input = format!("https://www.google.com/search?q={}", query);
        }
        self.quick_search_visible = false;
        self.quick_search_input.clear();
    }

    pub fn activate_snow(&mut self) {
        self.snow_active = true;
        self.snow_end_time = std::time::Instant::now() + std::time::Duration::from_secs(5);
    }

    pub fn current_tab(&self) -> Option<&Tab> {
        self.tabs.tabs.get(self.current_tab)
    }
}

pub fn view(browser: &BrowserState) -> Element<crate::Message> {
    let tabs_view = crate::tabs::view_tabs(browser);
    let address_bar = crate::tabs::view_address_bar(browser);
    let content_view = container(text("Web content here")).padding(10);

    let mut main_column = column![address_bar, tabs_view, content_view];

    if browser.quick_search_visible {
        main_column = main_column.push(crate::quick_search::view(&browser.quick_search_input));
    }

    let main_view = container(main_column)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .padding(0);

    let sidebar_view = crate::sidebar::view(browser);
    
    if matches!(browser.active_panel, Panel::None) {
        row![sidebar_view, main_view].into()
    } else {
        let panel_view = match browser.active_panel {
            Panel::Downloads => crate::downloads::view(&browser),
            Panel::History => crate::history::view(&browser),
            Panel::Modules => crate::modules_panel::view(&browser),
            Panel::Workflow => crate::workflow_panel::view(&browser),
            Panel::Keybindings => crate::keybindings_panel::view(&browser),
            Panel::Permissions => crate::permission_panel::view(&browser),
            Panel::Updater => crate::updater_panel::view(&browser.updater),  // Pass the updater field
            Panel::None => text("").into(),
        };
        row![sidebar_view, main_view, panel_view].into()
    }
}
