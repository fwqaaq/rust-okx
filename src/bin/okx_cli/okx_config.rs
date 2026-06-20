use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OkxProfile {
    pub api_key: String,
    pub secret_key: String,
    pub passphrase: String,
    pub demo: bool,
    pub base_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OkxConfig {
    pub default_profile: String,
    pub profiles: HashMap<String, OkxProfile>,
}

impl OkxConfig {
    pub fn config_path() -> PathBuf {
        okx_dir().join("config.toml")
    }

    pub fn load() -> Result<Option<Self>> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(None);
        }
        let text =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let config: Self =
            toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
        Ok(Some(config))
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
        }
        let text = toml::to_string_pretty(self)?;
        fs::write(&path, text)?;
        Ok(())
    }
}

fn okx_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_owned());
    PathBuf::from(home).join(".okx")
}
