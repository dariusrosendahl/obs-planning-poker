use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

const DEFAULT_PORT: u16 = 7777;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self { port: DEFAULT_PORT }
    }
}

fn config_path(config_dir: &PathBuf) -> PathBuf {
    config_dir.join("config.json")
}

pub fn load(config_dir: &PathBuf) -> AppConfig {
    let path = config_path(config_dir);
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save(config_dir: &PathBuf, config: &AppConfig) -> Result<(), String> {
    fs::create_dir_all(config_dir).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(config_path(config_dir), json).map_err(|e| e.to_string())
}
