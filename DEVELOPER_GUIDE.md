# HyprBrowser Module Development Guide

This guide provides comprehensive instructions for developing custom modules for HyprBrowser.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Module Structure](#module-structure)
3. [API Reference](#api-reference)
4. [Examples](#examples)
5. [Best Practices](#best-practices)
6. [Publishing](#publishing)
7. [Troubleshooting](#troubleshooting)

## Getting Started

### What is a Module?

A HyprBrowser module is a Rust plugin that extends the browser's functionality without requiring a rebuild. Modules can:

- Add custom UI panels
- Intercept keyboard input
- Access browser state (tabs, history, etc.)
- Store persistent configuration
- Integrate with external APIs

### Prerequisites

- Rust 1.70+ installed
- Basic Rust knowledge
- HyprBrowser installed (`cargo install hyprbrowser` or built from source)

### Module Naming Convention

All modules must follow this naming pattern:

```
hyprbrowser_mod_<your_module_name>.rs
```

Examples:

- `hyprbrowser_mod_tab_groups.rs`
- `hyprbrowser_mod_dark_mode.rs`
- `hyprbrowser_mod_quick_notes.rs`

## Module Structure

### Minimal Module

```rust
// hyprbrowser_mod_mymodule.rs

pub struct MyModule {
    pub name: String,
    pub version: String,
    pub enabled: bool,
}

impl MyModule {
    pub fn new() -> Self {
        MyModule {
            name: "My Module".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
        }
    }

    pub fn render_panel(&self) -> Element {
        text("Hello from My Module").into()
    }
}

pub fn init() -> MyModule {
    MyModule::new()
}
```

### Full-Featured Module

```rust
use iced::widget::*;
use iced::Element;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyModuleConfig {
    pub enabled: bool,
    pub setting1: String,
    pub setting2: bool,
}

pub struct MyModule {
    pub name: String,
    pub version: String,
    pub description: String,
    pub enabled: bool,
    config: MyModuleConfig,
}

impl MyModule {
    pub fn new() -> Self {
        MyModule {
            name: "My Module".to_string(),
            version: "1.0.0".to_string(),
            description: "A description of what this module does".to_string(),
            enabled: true,
            config: MyModuleConfig {
                enabled: true,
                setting1: "default".to_string(),
                setting2: false,
            },
        }
    }

    pub fn render_panel(&self) -> Element<'static, String> {
        column![
            text(&self.name).size(16),
            text(&self.description).size(12),
            row![
                text("Setting 1:").size(12),
                text_input("", &self.config.setting1, |_| "input".to_string()).padding(8),
            ].spacing(8),
            checkbox("Setting 2", self.config.setting2, |_| "toggle".to_string()),
        ]
        .spacing(12)
        .padding(16)
        .into()
    }

    pub fn on_key_press(&mut self, key: u32) -> Option<String> {
        // Handle keyboard events
        match key {
            // Example: handle custom shortcuts
            _ => None,
        }
    }

    pub fn on_message(&mut self, msg: String) {
        // Handle messages from UI events
        match msg.as_str() {
            "input" => {
                // Handle input change
            }
            "toggle" => {
                self.config.setting2 = !self.config.setting2;
            }
            _ => {}
        }
    }

    pub fn save_state(&self) -> anyhow::Result<()> {
        let config_dir = self.get_config_dir()?;
        std::fs::create_dir_all(&config_dir)?;

        let config_path = config_dir.join("config.json");
        let json = serde_json::to_string_pretty(&self.config)?;
        std::fs::write(config_path, json)?;

        Ok(())
    }

    pub fn load_state(&mut self) -> anyhow::Result<()> {
        let config_dir = self.get_config_dir()?;
        let config_path = config_dir.join("config.json");

        if config_path.exists() {
            let contents = std::fs::read_to_string(config_path)?;
            self.config = serde_json::from_str(&contents)?;
        }

        Ok(())
    }

    fn get_config_dir(&self) -> anyhow::Result<PathBuf> {
        let dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Cannot find data directory"))?
            .join("hyprbrowser")
            .join("modules")
            .join(&self.name.to_lowercase().replace(" ", "_"));

        Ok(dir)
    }
}

pub fn init() -> MyModule {
    let mut module = MyModule::new();
    let _ = module.load_state();
    module
}
```

## API Reference

### Core Module Traits

#### `render_panel(&self) -> Element<'static, String>`

Renders the UI panel for your module.

**Returns**: `iced::Element` - Your UI layout

**Example**:

```rust
pub fn render_panel(&self) -> Element<'static, String> {
    container(
        column![
            text(&self.name).size(14),
            button(text("Click Me")).on_press("button_clicked".to_string()),
        ]
        .spacing(8)
        .padding(12)
    )
    .padding(8)
    .into()
}
```

#### `on_key_press(&mut self, key: u32) -> Option<String>`

Handle keyboard input.

**Parameters**:

- `key: u32` - Key code (see key code reference below)

**Returns**: `Option<String>` - Optional message to send to UI handler

**Example**:

```rust
pub fn on_key_press(&mut self, key: u32) -> Option<String> {
    match key {
        // Shift+M
        77 if self.shift_pressed => Some("modal_toggle".to_string()),
        _ => None,
    }
}
```

#### `save_state(&self) -> anyhow::Result<()>`

Persist module state to disk.

**Returns**: `anyhow::Result<()>` - Success or error

**Example**:

```rust
pub fn save_state(&self) -> anyhow::Result<()> {
    let config_dir = self.get_config_dir()?;
    std::fs::create_dir_all(&config_dir)?;

    let path = config_dir.join("state.json");
    let json = serde_json::to_string_pretty(&self.data)?;
    std::fs::write(path, json)?;

    Ok(())
}
```

#### `load_state(&mut self) -> anyhow::Result<()>`

Load module state from disk.

**Example**:

```rust
pub fn load_state(&mut self) -> anyhow::Result<()> {
    let config_dir = self.get_config_dir()?;
    let path = config_dir.join("state.json");

    if path.exists() {
        let contents = std::fs::read_to_string(path)?;
        self.data = serde_json::from_str(&contents)?;
    }

    Ok(())
}
```

### Key Codes

Common keyboard keys:

| Key | Code | Key | Code |
|-----|------|-----|------|
| A-Z | 65-90 | 0-9 | 48-57 |
| Space | 32 | Enter | 13 |
| Tab | 9 | Escape | 27 |
| Backspace | 8 | Delete | 46 |
| Shift | 16 | Ctrl | 17 |
| Alt | 18 | F1-F12 | 112-123 |

Use modifier flags:

```rust
const SHIFT: u8 = 1;
const CTRL: u8 = 2;
const ALT: u8 = 4;
```

### Accessing Browser State

Modules can access:

- Current tab URL
- Open tabs list
- Browser history
- Downloads
- Settings

**Future API** (in development):

```rust
pub struct BrowserContext {
    pub current_tab: &Tab,
    pub all_tabs: Vec<&Tab>,
    pub history: Vec<HistoryEntry>,
    pub downloads: Vec<Download>,
}
```

## Examples

### Example 1: Quick Notes

```rust
// hyprbrowser_mod_quick_notes.rs

pub struct QuickNotes {
    pub name: String,
    pub notes: Vec<String>,
}

impl QuickNotes {
    pub fn new() -> Self {
        QuickNotes {
            name: "Quick Notes".to_string(),
            notes: vec![],
        }
    }

    pub fn render_panel(&self) -> Element<'static, String> {
        let mut col = column![text(&self.name).size(16)].spacing(8).padding(12);

        for (i, note) in self.notes.iter().enumerate() {
            col = col.push(
                row![
                    text(note).size(12),
                    button(text("✕")).on_press(format!("delete_{}", i)),
                ]
                .spacing(8)
            );
        }

        col = col.push(
            button(text("+ Add Note")).on_press("add_note".to_string())
        );

        container(col).into()
    }

    pub fn save_state(&self) -> anyhow::Result<()> {
        let path = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("No data dir"))?
            .join("hyprbrowser/modules/notes.json");

        std::fs::create_dir_all(path.parent().unwrap())?;
        let json = serde_json::to_string_pretty(&self.notes)?;
        std::fs::write(path, json)?;

        Ok(())
    }

    pub fn load_state(&mut self) -> anyhow::Result<()> {
        let path = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("No data dir"))?
            .join("hyprbrowser/modules/notes.json");

        if path.exists() {
            let contents = std::fs::read_to_string(path)?;
            self.notes = serde_json::from_str(&contents)?;
        }

        Ok(())
    }
}

pub fn init() -> QuickNotes {
    let mut module = QuickNotes::new();
    let _ = module.load_state();
    module
}
```

### Example 2: Custom Search Engine

```rust
// hyprbrowser_mod_duckduckgo.rs

pub struct DuckDuckGoSearch {
    pub name: String,
    pub enabled: bool,
}

impl DuckDuckGoSearch {
    pub fn new() -> Self {
        DuckDuckGoSearch {
            name: "DuckDuckGo Search".to_string(),
            enabled: true,
        }
    }

    pub fn render_panel(&self) -> Element<'static, String> {
        column![
            text(&self.name).size(14),
            text("Use DuckDuckGo for private searches").size(11),
            checkbox("Enabled", self.enabled, |_| "toggle".to_string()),
        ]
        .spacing(8)
        .padding(12)
        .into()
    }

    pub fn on_key_press(&mut self, key: u32) -> Option<String> {
        // Intercept search and redirect to DuckDuckGo
        None
    }
}

pub fn init() -> DuckDuckGoSearch {
    DuckDuckGoSearch::new()
}
```

## Best Practices

### 1. Error Handling

Always handle errors gracefully:

```rust
pub fn load_state(&mut self) -> anyhow::Result<()> {
    match self.read_config() {
        Ok(config) => {
            self.config = config;
            log::info!("Loaded configuration");
        }
        Err(e) => {
            log::warn!("Failed to load config, using defaults: {}", e);
            // Don't crash, just use defaults
        }
    }

    Ok(())
}
```

### 2. Performance

Keep modules lightweight:

```rust
// ❌ Bad: Heavy computation on render
pub fn render_panel(&self) -> Element {
    let expensive = (0..1_000_000).sum::<i32>();
    text(format!("Sum: {}", expensive)).into()
}

// ✅ Good: Cache heavy computations
pub fn render_panel(&self) -> Element {
    text(format!("Sum: {}", self.cached_sum)).into()
}
```

### 3. Storage

Use standard directories:

```rust
fn get_module_dir(&self) -> anyhow::Result<PathBuf> {
    let dir = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("No data dir"))?
        .join("hyprbrowser")
        .join("modules")
        .join(&self.name.to_lowercase().replace(" ", "_"));

    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}
```

### 4. Logging

Use the `log` crate:

```rust
use log::{info, warn, error, debug};

pub fn load_state(&mut self) -> anyhow::Result<()> {
    info!("Loading {} state...", self.name);
    // ... load logic ...
    info!("Successfully loaded state");
    Ok(())
}
```

### 5. Documentation

Document your module:

```rust
//! Quick Notes Module
//!
//! Allows users to save quick notes directly in the browser sidebar.
//!
//! ## Usage
//! - Click "Quick Notes" in the sidebar
//! - Type a note and press Enter
//! - Notes are auto-saved
//!
//! ## Keyboard Shortcuts
//! - Ctrl+N: New note
//! - Ctrl+D: Delete note

pub struct QuickNotes {
    // ...
}
```

## Publishing

### Step 1: Create GitHub Repository

```bash
mkdir hyprbrowser_mod_myfeature
cd hyprbrowser_mod_myfeature
git init
```

### Step 2: Add Files

```
hyprbrowser_mod_myfeature/
├── src/
│   └── lib.rs                  # Your module code
├── Cargo.toml                  # Dependencies (optional)
├── README.md                   # Documentation
├── LICENSE                     # MIT, Apache 2.0, etc.
└── .gitignore                 # Git ignore rules
```

### Step 3: Write README

```markdown
# HyprBrowser: My Feature

A cool feature for HyprBrowser!

## Installation

1. Open HyprBrowser
2. Go to Modules Panel (sidebar gear icon)
3. Search for "hyprbrowser_mod_myfeature"
4. Click Install

## Usage

- Open the sidebar and click "My Feature"
- Use the UI to configure
- Settings are auto-saved

## License

MIT License - See LICENSE file
```

### Step 4: Publish

```bash
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/yourusername/hyprbrowser_mod_myfeature.git
git push -u origin main
```

### Step 5: Make it Discoverable

The module will be automatically discovered via GitHub API search for:

```
hyprbrowser_mod_*
```

Modules Panel will show your module in search results!

## Troubleshooting

### Module Not Showing in Modules Panel

1. Check filename matches `hyprbrowser_mod_*.rs`
2. Verify file is in `data/modules/` directory
3. Check console (DevTools) for errors
4. Ensure `pub fn init()` exists

### State Not Persisting

1. Ensure `save_state()` returns `Ok(())`
2. Check data directory has write permissions
3. Verify path exists (use `create_dir_all`)

### UI Not Rendering

1. Ensure `render_panel()` returns `Element`
2. Use `container()` or `column!()` for layout
3. Check console for Rust panic messages
4. Test with simple `text()` first

### Performance Issues

1. Profile with `cargo flamegraph`
2. Avoid heavy computation in `render_panel()`
3. Cache expensive operations
4. Use `tokio::spawn` for async work

### GitHub Discovery Not Working

1. Repository name must be `hyprbrowser_mod_*`
2. Must be public
3. API rate limits apply (60 req/hour unauthenticated)
4. Fallback to manual upload

---

**Happy module development! 🚀**

For more help: [GitHub Discussions](https://github.com/hyperbrowser/hyprbrowser/discussions)
