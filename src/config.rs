use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct OpenerConfig {
    pub url: String,
    pub file: String,
    pub app: String,
}

impl Default for OpenerConfig {
    fn default() -> Self {
        Self {
            url: "default".into(),
            file: "default".into(),
            app: "default".into(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub opener: OpenerConfig,
}

impl Config {
    pub fn load() -> Self {
        let path = config_path();
        let Ok(content) = fs::read_to_string(&path) else {
            return Self::default();
        };
        toml::from_str(&content).unwrap_or_default()
    }
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("linker")
        .join("config.toml")
}
