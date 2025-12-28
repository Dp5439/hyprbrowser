#![allow(dead_code)]
#![allow(unused)]
#![allow(mismatched_lifetime_syntaxes)]
mod browser;
mod tabs;
mod quick_search;
mod snow;
mod permission_panel;
mod adblock;
mod vpn;
mod devtools;
mod theme;
mod downloads;
mod history;
mod sidebar;
mod icons;
mod workflow_panel;
mod keybindings_panel;
mod module_loader;
mod modules_panel;
mod updater_panel;
mod state;
mod utils;
mod gpu_detect;
mod gpu_benchmark;

use iced::{
    executor, widget::*, Application, Command, Element, Settings, Size, Theme,
};

fn main() -> iced::Result {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .try_init()
        .ok();

    let mut settings = Settings::default();
    settings.window.decorations = true;
    settings.window.transparent = false;
    settings.window.size = Size::new(1280.0, 720.0);
    
    // Load saved window state
    if let Ok(saved_state) = state::State::load() {
        if let Some((width, height)) = saved_state.window_size {
            settings.window.size = Size::new(width as f32, height as f32);
        }
    }

    HyprBrowser::run(settings)
}

pub struct HyprBrowser {
    browser_state: browser::BrowserState,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Browser
    BrowserMessage(browser::BrowserMessage),
    // GPU
    GpuDetected(gpu_detect::GpuInfo),
    // Tab operations
    NewTab,
    NewIncognitoTab,
    CloseTab(usize),
    CheckForUpdates,
    DownloadUpdate,
    SelectTab(usize),
    DuplicateTab,
    CloseOtherTabs,
    PinTab(usize),
    UnpinTab(usize),
    ToggleMultiPanel,
    // Quick search
    QuickSearchToggle,
    QuickSearchInput(String),
    QuickSearchExecute,
    // Theme
    ThemeChanged(theme::ThemeMode),
    // Panel toggles
    ToggleDownloads,
    ToggleHistory,
    ToggleModules,
    ToggleWorkflow,
    ToggleKeybindings,
    TogglePermissions,
    ToggleUpdater,
    // Adblock & VPN
    ToggleAdblock,
    ToggleVpn,
    // GPU Settings
    ToggleEffectIntensity,
    AdjustParticleCount(usize),
    // Keyboard
    KeyPressed(KeyboardInput),
    // Window
    WindowClosed,
    // Other
    Tick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyboardInput {
    pub key: u32,
    pub modifiers: u8, // bitfield: 1=shift, 2=ctrl, 4=alt
}

impl Application for HyprBrowser {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        // Create a dummy GPU info for now; will be replaced by GpuDetected message
        let dummy_gpu = gpu_detect::GpuInfo {
            tier: gpu_detect::GpuTier::LowPower,
            adapter_name: "Detecting...".to_string(),
            backend: wgpu::Backend::Empty, // Changed from Backends::PRIMARY to Backend::Empty
            max_texture_size: 2048,
            max_uniform_buffer_size: 65536,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        };

        let browser_state = browser::BrowserState::new(dummy_gpu);
        
        // Launch GPU detection task
        let cmd = Command::perform(
            async {
                gpu_detect::GpuInfo::detect().await
            },
            Message::GpuDetected,
        );

        (HyprBrowser { browser_state }, cmd)
    }

    fn title(&self) -> String {
        "HyprBrowser".to_string()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::GpuDetected(gpu_info) => {
                self.browser_state.gpu_info = gpu_info.clone();
                self.browser_state.gpu_settings = gpu_detect::GpuSettings::for_gpu(&gpu_info);
                log::info!("GPU detection complete: {}", gpu_info.to_string());
                Command::none()
            }
            Message::NewTab => {
                self.browser_state.new_tab();
                Command::none()
            }
            Message::NewIncognitoTab => {
                self.browser_state.new_incognito_tab();
                Command::none()
            }
            Message::CloseTab(idx) => {
                self.browser_state.close_tab(idx);
                Command::none()
            }
            Message::SelectTab(idx) => {
                self.browser_state.select_tab(idx);
                Command::none()
            }
            Message::DuplicateTab => {
                self.browser_state.duplicate_tab();
                Command::none()
            }
            Message::CloseOtherTabs => {
                self.browser_state.close_other_tabs();
                Command::none()
            }
            Message::PinTab(idx) => {
                self.browser_state.pin_tab(idx);
                Command::none()
            }
            Message::UnpinTab(idx) => {
                self.browser_state.unpin_tab(idx);
                Command::none()
            }
            Message::ToggleMultiPanel => {
                self.browser_state.toggle_multi_panel();
                Command::none()
            }
            Message::QuickSearchToggle => {
                self.browser_state.toggle_quick_search();
                Command::none()
            }
            Message::QuickSearchInput(input) => {
                self.browser_state.quick_search_input = input;
                Command::none()
            }
            Message::QuickSearchExecute => {
                self.browser_state.execute_quick_search();
                Command::none()
            }
            Message::ThemeChanged(mode) => {
                self.browser_state.theme_mode = mode;
                let _ = state::State::save_theme(&mode);
                Command::none()
            }
            Message::ToggleDownloads => {
                self.browser_state.active_panel = if self.browser_state.active_panel == browser::Panel::Downloads {
                    browser::Panel::None
                } else {
                    browser::Panel::Downloads
                };
                Command::none()
            }
            Message::ToggleHistory => {
                self.browser_state.active_panel = if self.browser_state.active_panel == browser::Panel::History {
                    browser::Panel::None
                } else {
                    browser::Panel::History
                };
                Command::none()
            }
            Message::ToggleModules => {
                self.browser_state.active_panel = if self.browser_state.active_panel == browser::Panel::Modules {
                    browser::Panel::None
                } else {
                    browser::Panel::Modules
                };
                Command::none()
            }
            Message::ToggleWorkflow => {
                self.browser_state.active_panel = if self.browser_state.active_panel == browser::Panel::Workflow {
                    browser::Panel::None
                } else {
                    browser::Panel::Workflow
                };
                Command::none()
            }
            Message::ToggleKeybindings => {
                self.browser_state.active_panel = if self.browser_state.active_panel == browser::Panel::Keybindings {
                    browser::Panel::None
                } else {
                    browser::Panel::Keybindings
                };
                Command::none()
            }
            Message::TogglePermissions => {
                self.browser_state.active_panel = if self.browser_state.active_panel == browser::Panel::Permissions {
                    browser::Panel::None
                } else {
                    browser::Panel::Permissions
                };
                Command::none()
            }
            Message::ToggleUpdater => {
                self.browser_state.active_panel = if self.browser_state.active_panel == browser::Panel::Updater {
                    browser::Panel::None
                } else {
                    browser::Panel::Updater
                };
                Command::none()
            }
            Message::ToggleAdblock => {
                self.browser_state.adblock_enabled = !self.browser_state.adblock_enabled;
                Command::none()
            }
            Message::ToggleVpn => {
                self.browser_state.vpn_enabled = !self.browser_state.vpn_enabled;
                Command::none()
            }
            Message::ToggleEffectIntensity => {
                // Cycle through intensity levels: 1.0 -> 0.6 -> 0.3 -> 1.0
                self.browser_state.gpu_settings.effect_intensity = match self.browser_state.gpu_settings.effect_intensity {
                    x if (x - 1.0).abs() < 0.01 => 0.6,
                    x if (x - 0.6).abs() < 0.01 => 0.3,
                    _ => 1.0,
                };
                log::info!(
                    "Effect Intensity changed to {:.0}%",
                    self.browser_state.gpu_settings.effect_intensity * 100.0
                );
                Command::none()
            }
            Message::AdjustParticleCount(count) => {
                self.browser_state.gpu_settings.particle_count = count;
                log::info!("Particle count adjusted to {}", count);
                Command::none()
            }
            Message::WindowClosed => {
                let _ = state::State::save_browser_state(&self.browser_state);
                std::process::exit(0);
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = browser::view(&self.browser_state);
        container(content).into()
    }

    fn theme(&self) -> Theme {
        match self.browser_state.theme_mode {
            theme::ThemeMode::Light => Theme::Light,
            theme::ThemeMode::Dark => Theme::Dark,
            theme::ThemeMode::System => {
                // Detect system theme
                #[cfg(target_os = "windows")]
                {
                    // Windows: check registry or use default
                    Theme::Dark
                }
                #[cfg(not(target_os = "windows"))]
                {
                    if dark_light::detect() == dark_light::Mode::Dark {
                        Theme::Dark
                    } else {
                        Theme::Light
                    }
                }
            }
        }
    }
}
