//! Authenticated Convert endpoints (`/api/v5/asset/convert/*`).

mod api;
mod requests;
mod responses;

pub use api::Convert;
pub use requests::{
    ConvertCurrenciesRequest, ConvertCurrencyPairRequest, ConvertHistoryRequest, ConvertMode,
    ConvertQuoteRequest, ConvertTradeRequest,
};
pub use responses::{
    ConvertCurrency, ConvertCurrencyPair, ConvertHistory, ConvertQuote, ConvertTradeResult,
    ConvertTradeState,
};

#[cfg(test)]
mod tests;
