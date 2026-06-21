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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub watchlist: Vec<String>,
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

    pub fn profile_watchlist_or_default(
        &self,
        profile_name: &str,
        default_watchlist: &[&str],
    ) -> Vec<String> {
        self.profiles
            .get(profile_name)
            .map(|profile| normalize_watchlist(&profile.watchlist, default_watchlist))
            .unwrap_or_else(|| {
                default_watchlist
                    .iter()
                    .map(|id| (*id).to_owned())
                    .collect()
            })
    }

    pub fn set_profile_watchlist(
        &mut self,
        profile_name: &str,
        watchlist: &[String],
    ) -> Result<()> {
        let Some(profile) = self.profiles.get_mut(profile_name) else {
            anyhow::bail!("profile `{profile_name}` does not exist");
        };
        profile.watchlist = normalize_watchlist(watchlist, &[]);
        Ok(())
    }

    pub fn save_profile_watchlist(profile_name: &str, watchlist: &[String]) -> Result<()> {
        let mut config = Self::load()?.context("missing ~/.okx/config.toml")?;
        config.set_profile_watchlist(profile_name, watchlist)?;
        config.save()
    }
}

fn okx_dir() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_owned());
    PathBuf::from(home).join(".okx")
}

fn normalize_watchlist(values: &[String], default_watchlist: &[&str]) -> Vec<String> {
    let source: Vec<String> = if values.is_empty() {
        default_watchlist
            .iter()
            .map(|id| (*id).to_owned())
            .collect()
    } else {
        values.to_vec()
    };
    let mut normalized = Vec::new();
    for inst_id in source {
        let inst_id = inst_id.trim().to_ascii_uppercase();
        if inst_id.is_empty() || normalized.iter().any(|existing| existing == &inst_id) {
            continue;
        }
        normalized.push(inst_id);
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile() -> OkxProfile {
        OkxProfile {
            api_key: "key".to_owned(),
            secret_key: "secret".to_owned(),
            passphrase: "pass".to_owned(),
            demo: true,
            base_url: None,
            watchlist: Vec::new(),
        }
    }

    #[test]
    fn old_profile_without_watchlist_deserializes() {
        let config: OkxConfig = toml::from_str(
            r#"
default_profile = "live"

[profiles.live]
api_key = "key"
secret_key = "secret"
passphrase = "pass"
demo = true
"#,
        )
        .unwrap();

        assert!(config.profiles["live"].watchlist.is_empty());
    }

    #[test]
    fn empty_profile_watchlist_uses_default() {
        let mut profiles = HashMap::new();
        profiles.insert("live".to_owned(), profile());
        let config = OkxConfig {
            default_profile: "live".to_owned(),
            profiles,
        };

        assert_eq!(
            config.profile_watchlist_or_default("live", &["BTC-USDT", "ETH-USDT"]),
            vec!["BTC-USDT".to_owned(), "ETH-USDT".to_owned()]
        );
    }

    #[test]
    fn profile_watchlist_roundtrips_normalized() {
        let mut profiles = HashMap::new();
        profiles.insert("live".to_owned(), profile());
        let mut config = OkxConfig {
            default_profile: "live".to_owned(),
            profiles,
        };

        config
            .set_profile_watchlist(
                "live",
                &[
                    "eth-usdt".to_owned(),
                    "ETH-USDT".to_owned(),
                    " sol-usdt ".to_owned(),
                ],
            )
            .unwrap();

        let text = toml::to_string_pretty(&config).unwrap();
        let parsed: OkxConfig = toml::from_str(&text).unwrap();
        assert_eq!(
            parsed.profile_watchlist_or_default("live", &["BTC-USDT"]),
            vec!["ETH-USDT".to_owned(), "SOL-USDT".to_owned()]
        );
    }
}
