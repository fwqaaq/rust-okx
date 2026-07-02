use std::collections::HashMap;
use std::io::{self, BufRead, Write};

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rust_okx::Credentials;

use crate::okx_config::{OkxConfig, OkxProfile};

pub const DEFAULT_PROFILE: &str = "live";

/// Load credentials from `~/.okx/config.toml`.
/// 默认使用 `[profiles.live]`，可通过 `--profile NAME` 指定其他 profile。
/// 文件或 profile 不存在时交互式输入并保存。
/// 返回 `(credentials, demo_flag, optional_base_url)`。
pub fn load_or_prompt(profile_name: Option<&str>) -> Result<(Credentials, bool, Option<String>)> {
    let existing = OkxConfig::load()?;
    let name = profile_name.unwrap_or(DEFAULT_PROFILE);

    if let Some(ref config) = existing {
        if let Some(profile) = config.profiles.get(name) {
            println!("已从 ~/.okx/config.toml 加载 profile `{name}` ✓");
            return Ok(profile_to_creds(profile));
        }

        let mut available: Vec<&str> = config.profiles.keys().map(String::as_str).collect();
        available.sort_unstable();
        anyhow::bail!(
            "profile `{name}` 不存在（可用: {}）\n提示：用 --profile 指定，或在 config.toml 中添加 [profiles.{name}]",
            available.join(", ")
        );
    }

    println!("未找到 ~/.okx/config.toml，请输入 OKX API 凭证：\n");

    // Interactive prompt — only reached when no profiles exist at all.
    let api_key = prompt_line("API Key:    ");
    let secret_key = prompt_line("API Secret: ");
    let passphrase = prompt_hidden("Passphrase: ");
    let demo_input = prompt_line("Demo 模式? (y/N): ");
    let demo = matches!(demo_input.to_ascii_lowercase().trim(), "y" | "yes");
    let base_url_input = prompt_line("Base URL [https://www.okx.com]: ");
    let base_url = if base_url_input.is_empty() {
        "https://www.okx.com".to_owned()
    } else {
        base_url_input
    };

    let new_profile = OkxProfile {
        api_key: api_key.clone(),
        secret_key: secret_key.clone(),
        passphrase: passphrase.clone(),
        demo,
        base_url: Some(base_url.clone()),
        watchlist: Vec::new(),
    };

    let mut config = existing.unwrap_or_else(|| OkxConfig {
        default_profile: name.to_owned(),
        profiles: HashMap::new(),
    });
    config.profiles.insert(name.to_owned(), new_profile);
    config.save()?;
    println!("\nProfile `{name}` 已保存至 ~/.okx/config.toml ✓");

    Ok((
        Credentials::new(api_key, secret_key, passphrase),
        demo,
        Some(base_url),
    ))
}

fn profile_to_creds(profile: &OkxProfile) -> (Credentials, bool, Option<String>) {
    let creds = Credentials::new(
        profile.api_key.clone(),
        profile.secret_key.clone(),
        profile.passphrase.clone(),
    );
    (creds, profile.demo, profile.base_url.clone())
}

pub fn prompt_line(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().ok();
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).ok();
    line.trim().to_owned()
}

fn prompt_hidden(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().ok();

    enable_raw_mode().unwrap();
    let mut input = String::new();
    loop {
        if let Ok(Event::Key(key)) = read() {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match key.code {
                KeyCode::Enter => break,
                KeyCode::Char(c) => {
                    input.push(c);
                    print!("*");
                    io::stdout().flush().ok();
                }
                KeyCode::Backspace if input.pop().is_some() => {
                    print!("\x08 \x08");
                    io::stdout().flush().ok();
                }
                _ => {}
            }
        }
    }
    disable_raw_mode().unwrap();
    print!("\r\n");
    io::stdout().flush().ok();
    input
}
