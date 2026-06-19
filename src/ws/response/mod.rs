//! Typed top-level OKX WebSocket response bodies.
//!
//! [`crate::ws::WsEvent`] keeps channel and operation data as raw bytes until a
//! caller chooses a row type. These envelopes support direct Serde decoding.

mod acknowledgement;
mod operation;
mod push;
mod spread;

use std::collections::BTreeMap;

use serde_json::Value;

/// Unrecognized top-level response fields retained for forward compatibility.
pub type ResponseExtraFields = BTreeMap<String, Value>;

pub use acknowledgement::{
    ChannelAcknowledgement, ChannelConnectionCountResponse, LoginAcknowledgement, NoticeResponse,
};
pub use operation::OperationResponse;
pub use push::PushResponse;
pub use spread::{
    AmendSpreadOrderResponse, CancelSpreadOrderResponse, MassCancelSpreadOrdersResponse,
    PlaceSpreadOrderResponse,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ws::model::TickerUpdate;

    #[test]
    fn parses_complete_channel_push_envelope() {
        let response: PushResponse<TickerUpdate> = serde_json::from_str(
            r#"{"arg":{"channel":"tickers","instId":"BTC-USDT"},"data":[{"instType":"SPOT","instId":"BTC-USDT","last":"1","ts":"2"}]}"#,
        )
        .unwrap();
        assert_eq!(response.arg.channel, "tickers");
        assert_eq!(response.data[0].inst_id, "BTC-USDT");
    }

    #[test]
    fn parses_login_acknowledgement_metadata() {
        let response: LoginAcknowledgement =
            serde_json::from_str(r#"{"event":"login","code":"0","msg":"","connId":"abc"}"#)
                .unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.conn_id, "abc");
    }
}
