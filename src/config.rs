use crate::app::{Language, TestMode, TextSource};
use crate::theme::Theme;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub test_mode: Option<TestMode>,
    pub punctuation: Option<bool>,
    pub text_source: Option<TextSource>,
    pub language: Option<Language>,
    pub blind_mode: Option<bool>,
    pub stop_on_error: Option<bool>,
    pub theme: Option<Theme>,
    pub sound_enabled: Option<bool>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            Ok(serde_json::from_str(&content).unwrap_or_default())
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not find config directory"))?;
        let app_dir = config_dir.join("wpm-rs");
        std::fs::create_dir_all(&app_dir)?;
        Ok(app_dir.join("config.json"))
    }
}
