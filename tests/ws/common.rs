use std::env;

use rust_okx::Credentials;

pub fn credentials() -> Option<Credentials> {
    let _ = dotenvy::dotenv();
    Some(Credentials::new(
        non_empty("OKX_API_KEY")?,
        non_empty("OKX_API_SECRET")?,
        non_empty("OKX_PASSPHRASE")?,
    ))
}

pub fn non_empty(var: &str) -> Option<String> {
    env::var(var).ok().filter(|value| !value.is_empty())
}
