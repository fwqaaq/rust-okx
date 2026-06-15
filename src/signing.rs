//! Request signing helpers (internal).
//!
//! OKX signs requests with `base64(HMAC_SHA256(secret, prehash))` where
//! `prehash = timestamp + method + requestPath + body`. See
//! <https://www.okx.com/docs-v5/en/#overview-rest-authentication>.

use base64::Engine;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use time::OffsetDateTime;

/// Format the current UTC time as an OKX timestamp, e.g. `2020-12-08T09:08:57.715Z`.
pub(crate) fn timestamp() -> String {
    format_timestamp(OffsetDateTime::now_utc())
}

fn format_timestamp(dt: OffsetDateTime) -> String {
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        dt.year(),
        u8::from(dt.month()),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second(),
        dt.millisecond(),
    )
}

/// Build the pre-hash string that is fed into the HMAC.
pub(crate) fn pre_hash(timestamp: &str, method: &str, request_path: &str, body: &str) -> String {
    format!("{timestamp}{method}{request_path}{body}")
}

/// Compute `base64(HMAC_SHA256(secret, message))`.
pub(crate) fn sign(message: &str, secret: &str) -> String {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret.as_bytes()).expect("HMAC accepts keys of any length");
    mac.update(message.as_bytes());
    base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamp_has_millisecond_precision_and_z_suffix() {
        let dt = OffsetDateTime::from_unix_timestamp(1_597_026_383)
            .unwrap()
            .replace_millisecond(85)
            .unwrap();
        assert_eq!(format_timestamp(dt), "2020-08-10T02:26:23.085Z");
    }

    #[test]
    fn pre_hash_concatenates_in_order() {
        let h = pre_hash(
            "2020-12-08T09:08:57.715Z",
            "GET",
            "/api/v5/account/balance",
            "",
        );
        assert_eq!(h, "2020-12-08T09:08:57.715ZGET/api/v5/account/balance");
    }

    #[test]
    fn sign_matches_known_vector() {
        // Reference value computed with:
        //   printf '%s' hello | openssl dgst -sha256 -hmac secret -binary | base64
        let sig = sign("hello", "secret");
        assert_eq!(sig, "iKqz7ejTrflNJquQ07r9SiCDBww7zOnAFO4EpEOEfAs=");
    }
}
