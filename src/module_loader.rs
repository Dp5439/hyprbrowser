use std::path::{Path, PathBuf};
use tokio::fs;
use reqwest::Client;

pub struct ModuleLoader {
    pub modules_dir: PathBuf,
}

impl ModuleLoader {
    pub fn new() -> Self {
        let modules_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("hyprbrowser")
            .join("modules");

        ModuleLoader { modules_dir }
    }

    pub async fn load_module(&self, name: &str) -> anyhow::Result<()> {
        // Ensure modules directory exists
        fs::create_dir_all(&self.modules_dir).await?;

        // Check if module exists locally
        let module_path = self.modules_dir.join(format!("{}.rs", name));
        if !module_path.exists() {
            // Try to fetch from GitHub
            self.download_from_github(name).await?;
        }

        Ok(())
    }

    async fn download_from_github(&self, module_name: &str) -> anyhow::Result<()> {
        let _client = Client::new();
        let _url = format!(
            "https://raw.githubusercontent.com/search?q=hyprbrowser_mod_{}&type=repositories",
            module_name
        );

        // In production, implement actual GitHub API search with rate-limit fallback
        log::info!("Fetching module: {}", module_name);

        Ok(())
    }

    pub fn get_installed_modules(&self) -> anyhow::Result<Vec<String>> {
        let mut modules = Vec::new();

        if !self.modules_dir.exists() {
            return Ok(modules);
        }

        for entry in std::fs::read_dir(&self.modules_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "rs") {
                if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                    modules.push(name.to_string());
                }
            }
        }

        Ok(modules)
    }

    pub fn uninstall_module(&self, name: &str) -> anyhow::Result<()> {
        let module_path = self.modules_dir.join(format!("{}.rs", name));
        if module_path.exists() {
            std::fs::remove_file(module_path)?;
        }
        Ok(())
    }

    pub async fn upload_module(&self, path: &Path) -> anyhow::Result<()> {
        let filename = path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;

        let dest = self.modules_dir.join(filename);
        tokio::fs::copy(path, dest).await?;

        Ok(())
    }

    pub fn compile_module(&self, name: &str) -> anyhow::Result<()> {
        let _module_path = self.modules_dir.join(format!("{}.rs", name));

        // In production, use rustc or cargo to compile
        log::info!("Compiling module: {}", name);

        Ok(())
    }
}
