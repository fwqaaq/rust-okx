//! Authenticated Convert endpoints (`/api/v5/asset/convert/*`).

mod api;
mod requests;
mod responses;

pub use api::Convert;
pub use requests::{ConvertHistoryRequest, ConvertQuoteRequest, ConvertTradeRequest};
pub use responses::{
    ConvertCurrency, ConvertCurrencyPair, ConvertHistory, ConvertQuote, ConvertTradeResult,
};

#[cfg(test)]
mod tests;
