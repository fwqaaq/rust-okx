use serde::Serialize;

/// Query parameters for historical/index/mark-price candlestick endpoints.
#[derive(Debug, Clone, Serialize)]
pub struct CandlesticksRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl CandlesticksRequest {
    /// Create a candlestick query for an instrument.
    pub fn new(inst_id: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            after: None,
            before: None,
            bar: None,
            limit: None,
        }
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the bar size, e.g. `1m`, `1H`, or `1D`.
    pub fn bar(mut self, bar: impl Into<String>) -> Self {
        self.bar = Some(bar.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for historical trades.
#[derive(Debug, Clone, Serialize)]
pub struct HistoryTradesRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    trade_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl HistoryTradesRequest {
    /// Create a historical trades query for an instrument.
    pub fn new(inst_id: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            trade_type: None,
            after: None,
            before: None,
            limit: None,
        }
    }

    /// Set the OKX trade type filter.
    pub fn trade_type(mut self, trade_type: impl Into<String>) -> Self {
        self.trade_type = Some(trade_type.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}
