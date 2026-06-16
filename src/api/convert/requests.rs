use crate::model::RequestParams;

/// Request body for `Convert::estimate_quote`.
pub type ConvertQuoteRequest = RequestParams;

/// Request body for `Convert::convert_trade`.
pub type ConvertTradeRequest = RequestParams;

/// Query parameters for `Convert::get_convert_history`.
pub type ConvertHistoryRequest = RequestParams;

/// Query parameters for `Convert::get_currencies`
pub type ConvertCurrencyRequest = RequestParams;

/// Query parameter for `Convert::get_currency_pair`
pub type ConvertCurrencyPairRequest = RequestParams;
