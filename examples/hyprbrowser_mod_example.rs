use iced::widget::{column, row, container, text, checkbox};
use iced::{Element, Alignment, Length};
use serde_json::json;
use anyhow::Result;

pub struct ExampleModule {
    pub name: String,
    pub version: String,
    pub description: String,
    pub enabled: bool,
    pub dark_mode: bool,
}

impl ExampleModule {
    pub fn new() -> Self {
        ExampleModule {
            name: "Example Module".to_string(),
            version: "1.0.0".to_string(),
            description: "Example module demonstrating custom panel and settings".to_string(),
            enabled: true,
            dark_mode: false,
        }
    }

    pub fn render_panel(&self) -> Element<'static, String> {
        let toggle = row![
            text("Dark Mode").size(12),
            checkbox("Enable dark theme", self.dark_mode),
        ]
        .spacing(8)
        .align_items(Alignment::Center);

        let info = column![
            text("Example Module").size(14),
            text(format!("v{}", self.version)).size(10),
            text(&self.description).size(11),
            toggle,
        ]
        .spacing(8)
        .padding(12);

        container(info)
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(8)
            .into()
    }

    pub fn on_key_press(&mut self, _key: u32) -> Option<String> {
        None
    }

    pub fn save_state(&self) -> Result<()> {
        let config = json!({
            "enabled": self.enabled,
            "dark_mode": self.dark_mode,
        });

        let config_path = std::path::PathBuf::from("data/modules/example.json");
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(config_path, serde_json::to_string_pretty(&config)?)?;

        Ok(())
    }

    pub fn load_state(&mut self) -> Result<()> {
        let config_path = std::path::PathBuf::from("data/modules/example.json");
        if config_path.exists() {
            let contents = std::fs::read_to_string(config_path)?;
            if let Ok(config) = serde_json::from_str::<serde_json::Value>(&contents) {
                if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
                    self.enabled = enabled;
                }
                if let Some(dark_mode) = config.get("dark_mode").and_then(|v| v.as_bool()) {
                    self.dark_mode = dark_mode;
                }
            }
        }
        Ok(())
    }
}

pub fn init() -> ExampleModule {
    ExampleModule::new()
}

fn main() {
    let mut module = init();
    if let Err(e) = module.load_state() {
        eprintln!("failed to load module state: {}", e);
    }
    let _panel: Element<'static, String> = module.render_panel();
    if let Err(e) = module.save_state() {
        eprintln!("failed to save module state: {}", e);
    }
    println!("ExampleModule initialized. dark_mode={}", module.dark_mode);
}
