use std::env;
use std::io::{self, BufRead, Write};

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEventKind, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rust_okx::Credentials;

/// Load credentials from environment variables (populated by `.env`).
/// If any of the three are missing, show a login prompt for all three.
pub fn load_or_prompt() -> Result<Credentials> {
    let key = env::var("OKX_API_KEY").ok();
    let secret = env::var("OKX_API_SECRET").ok();
    let pass = env::var("OKX_PASSPHRASE").ok();

    match (key, secret, pass) {
        (Some(k), Some(s), Some(p)) => {
            println!("已从 .env 加载 OKX 凭证 ✓");
            Ok(Credentials::new(k, s, p))
        }
        _ => {
            println!("未在 .env 中找到完整的 OKX 凭证，请手动输入：");
            println!("（提示：在项目根目录创建 .env 文件可跳过此步骤）\n");
            let k = prompt_line("API Key:     ");
            let s = prompt_line("API Secret:  ");
            let p = prompt_hidden("Passphrase:  ");
            Ok(Credentials::new(k, s, p))
        }
    }
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
                KeyCode::Backspace => {
                    if input.pop().is_some() {
                        print!("\x08 \x08");
                        io::stdout().flush().ok();
                    }
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
