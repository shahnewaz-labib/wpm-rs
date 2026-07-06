use crate::app::TestMode;
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const MAX_ENTRIES: usize = 200;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub wpm: f64,
    #[serde(default)]
    pub raw_wpm: f64,
    pub accuracy: f64,
    pub elapsed_secs: f64,
    pub test_mode: TestMode,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryData {
    pub entries: Vec<HistoryEntry>,
}

pub struct History {
    data: HistoryData,
    path: PathBuf,
}

impl History {
    pub fn load() -> Result<Self> {
        let path = Self::history_path()?;
        let data = if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HistoryData::default()
        };
        Ok(Self { data, path })
    }

    fn history_path() -> Result<PathBuf> {
        let data_dir = dirs::data_dir()
            .ok_or_else(|| color_eyre::eyre::eyre!("Could not find data directory"))?;
        let app_dir = data_dir.join("wpm-rs");
        std::fs::create_dir_all(&app_dir)?;
        Ok(app_dir.join("history.json"))
    }

    pub fn add_entry(&mut self, entry: HistoryEntry) {
        self.data.entries.push(entry);
        if self.data.entries.len() > MAX_ENTRIES {
            let overflow = self.data.entries.len() - MAX_ENTRIES;
            self.data.entries.drain(0..overflow);
        }
        let _ = self.save();
    }

    fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.data)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }

    pub fn best_wpm(&self, mode: TestMode) -> Option<f64> {
        self.data
            .entries
            .iter()
            .filter(|e| e.test_mode == mode)
            .map(|e| e.wpm)
            .fold(None, |acc, wpm| Some(acc.map_or(wpm, |best: f64| best.max(wpm))))
    }

    #[allow(dead_code)]
    pub fn entry_count(&self) -> usize {
        self.data.entries.len()
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.data.entries
    }
}
