use std::env;

use anyhow::{Context, Result};
use clap::Parser;
use rust_okx::OkxRegion;

#[derive(Debug, Parser)]
#[command(name = "okx-cli", about = "Realtime OKX terminal trading workspace")]
pub struct CliArgs {
    /// Instrument ID, e.g. BTC-USDT.
    #[arg(long)]
    pub inst: Option<String>,
    /// Candlestick bar, e.g. 1m, 5m, 15m, 1H, 4H, 1D.
    #[arg(long)]
    pub bar: Option<String>,
    /// Use OKX demo trading endpoints.
    #[arg(long)]
    pub demo: bool,
    /// OKX region: global, us, au, eea, eu.
    #[arg(long)]
    pub region: Option<String>,
    /// Enable real trading actions in the TUI.
    #[arg(long)]
    pub trade_enabled: bool,
    /// REST refresh interval in milliseconds.
    #[arg(long)]
    pub refresh_ms: Option<u64>,
    /// Profile name from ~/.okx/config.toml (defaults to default_profile).
    #[arg(long)]
    pub profile: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    pub inst_id: String,
    pub bar: String,
    pub demo: bool,
    pub region: OkxRegion,
    pub region_label: String,
    pub trade_enabled: bool,
    pub refresh_ms: u64,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            inst_id: "BTC-USDT".to_owned(),
            bar: "1m".to_owned(),
            demo: false,
            region: OkxRegion::Global,
            region_label: "global".to_owned(),
            trade_enabled: false,
            refresh_ms: 10_000,
        }
    }
}

impl RuntimeConfig {
    pub fn from_args(args: CliArgs) -> Result<Self> {
        let default = Self::default();

        let inst_id = args
            .inst
            .or_else(|| non_empty_env("OKX_CLI_INST"))
            .unwrap_or(default.inst_id);
        let bar = args
            .bar
            .or_else(|| non_empty_env("OKX_CLI_BAR"))
            .unwrap_or(default.bar);
        let region_label = args
            .region
            .or_else(|| non_empty_env("OKX_REGION"))
            .unwrap_or(default.region_label);
        let region = parse_region(&region_label)?;
        let refresh_ms = args
            .refresh_ms
            .or_else(|| {
                non_empty_env("OKX_CLI_REFRESH_MS").and_then(|value| value.parse::<u64>().ok())
            })
            .unwrap_or(default.refresh_ms)
            .max(1_000);
        let demo = args.demo || env_flag("OKX_DEMO_TRADING") || env_flag("OKX_CLI_DEMO");
        let trade_enabled = args.trade_enabled || env_flag("OKX_CLI_TRADE_ENABLED");

        Ok(Self {
            inst_id,
            bar,
            demo,
            region,
            region_label,
            trade_enabled,
            refresh_ms,
        })
    }

    pub fn mode_label(&self) -> &'static str {
        if self.demo { "DEMO" } else { "LIVE" }
    }
}

fn parse_region(value: &str) -> Result<OkxRegion> {
    match value.to_ascii_lowercase().as_str() {
        "global" => Ok(OkxRegion::Global),
        "us" | "au" => Ok(OkxRegion::Us),
        "eea" | "eu" => Ok(OkxRegion::Eea),
        other => Err(anyhow::anyhow!(
            "OKX region must be global, us, au, eea, or eu; got {other}"
        )),
    }
}

fn non_empty_env(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.trim().is_empty())
}

fn env_flag(name: &str) -> bool {
    matches!(
        env::var(name).as_deref(),
        Ok("1") | Ok("true") | Ok("TRUE") | Ok("yes") | Ok("YES")
    )
}

pub fn validate_bar(bar: &str) -> Result<()> {
    if crate::app::BAR_OPTIONS.contains(&bar) {
        Ok(())
    } else {
        Err(anyhow::anyhow!("unsupported bar `{bar}`"))
            .with_context(|| format!("supported bars: {}", crate::app::BAR_OPTIONS.join(", ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_args_override_defaults() {
        let cfg = RuntimeConfig::from_args(CliArgs {
            inst: Some("ETH-USDT".to_owned()),
            bar: Some("5m".to_owned()),
            demo: true,
            region: Some("us".to_owned()),
            trade_enabled: true,
            refresh_ms: Some(5_000),
            profile: None,
        })
        .unwrap();

        assert_eq!(cfg.inst_id, "ETH-USDT");
        assert_eq!(cfg.bar, "5m");
        assert!(cfg.demo);
        assert!(cfg.trade_enabled);
        assert_eq!(cfg.refresh_ms, 5_000);
    }

    #[test]
    fn refresh_interval_has_floor() {
        let cfg = RuntimeConfig::from_args(CliArgs {
            inst: None,
            bar: None,
            demo: false,
            region: None,
            trade_enabled: false,
            refresh_ms: Some(100),
            profile: None,
        })
        .unwrap();

        assert_eq!(cfg.refresh_ms, 1_000);
    }
}
