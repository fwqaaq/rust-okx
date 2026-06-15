//! Public reference-data endpoints (`/api/v5/public/*`).

use serde::{Deserialize, Serialize};

use crate::client::OkxClient;
use crate::error::Error;
use crate::model::{InstType, NumberString};
use crate::transport::Transport;

const INSTRUMENTS: &str = "/api/v5/public/instruments";
const SYSTEM_TIME: &str = "/api/v5/public/time";
const OPEN_INTEREST: &str = "/api/v5/public/open-interest";
const FUNDING_RATE: &str = "/api/v5/public/funding-rate";
const FUNDING_RATE_HISTORY: &str = "/api/v5/public/funding-rate-history";
const PRICE_LIMIT: &str = "/api/v5/public/price-limit";
const MARK_PRICE: &str = "/api/v5/public/mark-price";
const DELIVERY_EXERCISE_HISTORY: &str = "/api/v5/public/delivery-exercise-history";
const POSITION_TIERS: &str = "/api/v5/public/position-tiers";
const UNDERLYING: &str = "/api/v5/public/underlying";
const INSURANCE_FUND: &str = "/api/v5/public/insurance-fund";
const CONVERT_CONTRACT_COIN: &str = "/api/v5/public/convert-contract-coin";

/// Accessor for the public reference-data endpoints.
///
/// Obtain one via [`OkxClient::public_data`](crate::OkxClient::public_data).
pub struct PublicData<'a, T> {
    client: &'a OkxClient<T>,
}

impl<'a, T: Transport> PublicData<'a, T> {
    pub(crate) fn new(client: &'a OkxClient<T>) -> Self {
        Self { client }
    }

    /// Retrieve the list of tradable instruments.
    ///
    /// `GET /api/v5/public/instruments`
    ///
    /// `inst_family` is required for `FUTURES`, `SWAP`, and `OPTION` and ignored
    /// for `SPOT`/`MARGIN`. This endpoint is public (unauthenticated).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] if OKX rejects the request, or
    /// [`Error::Transport`]/[`Error::Decode`] on transport/parsing failure.
    pub async fn get_instruments(
        &self,
        inst_type: InstType,
        inst_family: Option<&str>,
    ) -> Result<Vec<Instrument>, Error> {
        let query = InstrumentsQuery {
            inst_type: &inst_type,
            inst_family,
        };
        self.client.get(INSTRUMENTS, &query, false).await
    }

    /// Retrieve OKX system time.
    ///
    /// `GET /api/v5/public/time`. Public.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Api`] if OKX rejects the request, or transport/decode
    /// errors.
    pub async fn get_system_time(&self) -> Result<Vec<SystemTime>, Error> {
        self.client.get(SYSTEM_TIME, &NoQuery, false).await
    }

    /// Retrieve open interest.
    ///
    /// `GET /api/v5/public/open-interest`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_open_interest(
        &self,
        request: &InstrumentFamilyRequest,
    ) -> Result<Vec<OpenInterest>, Error> {
        self.client.get(OPEN_INTEREST, request, false).await
    }

    /// Retrieve the current funding rate for a derivatives instrument.
    ///
    /// `GET /api/v5/public/funding-rate`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_funding_rate(&self, inst_id: &str) -> Result<Vec<FundingRate>, Error> {
        let query = InstIdQuery { inst_id };
        self.client.get(FUNDING_RATE, &query, false).await
    }

    /// Retrieve historical funding rates.
    ///
    /// `GET /api/v5/public/funding-rate-history`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_funding_rate_history(
        &self,
        request: &FundingRateHistoryRequest,
    ) -> Result<Vec<FundingRateHistory>, Error> {
        self.client
            .get(FUNDING_RATE_HISTORY, request, false)
            .await
    }

    /// Retrieve the price limit for an instrument.
    ///
    /// `GET /api/v5/public/price-limit`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_price_limit(&self, inst_id: &str) -> Result<Vec<PriceLimit>, Error> {
        let query = InstIdQuery { inst_id };
        self.client.get(PRICE_LIMIT, &query, false).await
    }

    /// Retrieve mark prices.
    ///
    /// `GET /api/v5/public/mark-price`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_mark_price(
        &self,
        request: &InstrumentFamilyRequest,
    ) -> Result<Vec<MarkPrice>, Error> {
        self.client.get(MARK_PRICE, request, false).await
    }

    /// Retrieve delivery/exercise history.
    ///
    /// `GET /api/v5/public/delivery-exercise-history`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_delivery_exercise_history(
        &self,
        request: &DeliveryExerciseHistoryRequest,
    ) -> Result<Vec<DeliveryExercise>, Error> {
        self.client
            .get(DELIVERY_EXERCISE_HISTORY, request, false)
            .await
    }

    /// Retrieve position tiers.
    ///
    /// `GET /api/v5/public/position-tiers`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_position_tiers(
        &self,
        request: &PositionTiersRequest,
    ) -> Result<Vec<PositionTier>, Error> {
        self.client.get(POSITION_TIERS, request, false).await
    }

    /// Retrieve underlying values for an instrument type.
    ///
    /// `GET /api/v5/public/underlying`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_underlying(&self, inst_type: InstType) -> Result<Vec<String>, Error> {
        let query = UnderlyingQuery {
            inst_type: &inst_type,
        };
        self.client.get(UNDERLYING, &query, false).await
    }

    /// Retrieve insurance-fund snapshots.
    ///
    /// `GET /api/v5/public/insurance-fund`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_insurance_fund(
        &self,
        request: &InsuranceFundRequest,
    ) -> Result<Vec<InsuranceFund>, Error> {
        self.client.get(INSURANCE_FUND, request, false).await
    }

    /// Convert between contract count and coin amount.
    ///
    /// `GET /api/v5/public/convert-contract-coin`. Public.
    ///
    /// # Errors
    ///
    /// See [`get_system_time`](Self::get_system_time).
    pub async fn get_convert_contract_coin(
        &self,
        request: &ConvertContractCoinRequest,
    ) -> Result<Vec<ConvertContractCoin>, Error> {
        self.client.get(CONVERT_CONTRACT_COIN, request, false).await
    }
}

#[derive(Serialize)]
struct NoQuery;

#[derive(Serialize)]
struct InstrumentsQuery<'a> {
    #[serde(rename = "instType")]
    inst_type: &'a InstType,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<&'a str>,
}

#[derive(Serialize)]
struct InstIdQuery<'a> {
    #[serde(rename = "instId")]
    inst_id: &'a str,
}

#[derive(Serialize)]
struct UnderlyingQuery<'a> {
    #[serde(rename = "instType")]
    inst_type: &'a InstType,
}

/// Query parameters for public endpoints filtered by instrument family.
#[derive(Debug, Clone, Serialize)]
pub struct InstrumentFamilyRequest {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
}

impl InstrumentFamilyRequest {
    /// Create a query for an instrument type.
    pub fn new(inst_type: InstType) -> Self {
        Self {
            inst_type,
            underlying: None,
            inst_id: None,
            inst_family: None,
        }
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for funding-rate history.
#[derive(Debug, Clone, Serialize)]
pub struct FundingRateHistoryRequest {
    #[serde(rename = "instId")]
    inst_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl FundingRateHistoryRequest {
    /// Create a funding-rate history query.
    pub fn new(inst_id: impl Into<String>) -> Self {
        Self {
            inst_id: inst_id.into(),
            after: None,
            before: None,
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

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Query parameters for delivery/exercise history.
#[derive(Debug, Clone, Serialize)]
pub struct DeliveryExerciseHistoryRequest {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl DeliveryExerciseHistoryRequest {
    /// Create a delivery/exercise history query.
    pub fn new(inst_type: InstType) -> Self {
        Self {
            inst_type,
            underlying: None,
            inst_family: None,
            after: None,
            before: None,
            limit: None,
        }
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
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

/// Query parameters for public position tiers.
#[derive(Debug, Clone, Serialize)]
pub struct PositionTiersRequest {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "tdMode")]
    td_mode: String,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tier: Option<String>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
}

impl PositionTiersRequest {
    /// Create a position-tiers query.
    pub fn new(inst_type: InstType, td_mode: impl Into<String>) -> Self {
        Self {
            inst_type,
            td_mode: td_mode.into(),
            underlying: None,
            inst_id: None,
            ccy: None,
            tier: None,
            inst_family: None,
        }
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the instrument ID filter.
    pub fn inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Set the tier filter.
    pub fn tier(mut self, tier: impl Into<String>) -> Self {
        self.tier = Some(tier.into());
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for insurance fund snapshots.
#[derive(Debug, Clone, Serialize)]
pub struct InsuranceFundRequest {
    #[serde(rename = "instType")]
    inst_type: InstType,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    fund_type: Option<String>,
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    underlying: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ccy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    inst_family: Option<String>,
}

impl InsuranceFundRequest {
    /// Create an insurance fund query.
    pub fn new(inst_type: InstType) -> Self {
        Self {
            inst_type,
            fund_type: None,
            underlying: None,
            ccy: None,
            before: None,
            after: None,
            limit: None,
            inst_family: None,
        }
    }

    /// Set the OKX fund type filter.
    pub fn fund_type(mut self, fund_type: impl Into<String>) -> Self {
        self.fund_type = Some(fund_type.into());
        self
    }

    /// Set the underlying filter.
    pub fn underlying(mut self, underlying: impl Into<String>) -> Self {
        self.underlying = Some(underlying.into());
        self
    }

    /// Set the currency filter.
    pub fn currency(mut self, ccy: impl Into<String>) -> Self {
        self.ccy = Some(ccy.into());
        self
    }

    /// Return records before this pagination cursor.
    pub fn before(mut self, before: impl Into<String>) -> Self {
        self.before = Some(before.into());
        self
    }

    /// Return records after this pagination cursor.
    pub fn after(mut self, after: impl Into<String>) -> Self {
        self.after = Some(after.into());
        self
    }

    /// Set the maximum number of rows to return.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the instrument family filter.
    pub fn inst_family(mut self, inst_family: impl Into<String>) -> Self {
        self.inst_family = Some(inst_family.into());
        self
    }
}

/// Query parameters for contract/coin conversion.
#[derive(Debug, Clone, Serialize)]
pub struct ConvertContractCoinRequest {
    #[serde(rename = "type")]
    conversion_type: String,
    #[serde(rename = "instId")]
    inst_id: String,
    sz: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    px: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<String>,
}

impl ConvertContractCoinRequest {
    /// Create a contract/coin conversion query.
    pub fn new(
        conversion_type: impl Into<String>,
        inst_id: impl Into<String>,
        sz: impl Into<String>,
    ) -> Self {
        Self {
            conversion_type: conversion_type.into(),
            inst_id: inst_id.into(),
            sz: sz.into(),
            px: None,
            unit: None,
        }
    }

    /// Set the price used for conversion.
    pub fn price(mut self, px: impl Into<String>) -> Self {
        self.px = Some(px.into());
        self
    }

    /// Set the unit used for conversion.
    pub fn unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}

/// A tradable instrument.
///
/// Only commonly used fields are modeled; the struct is `#[non_exhaustive]` and
/// unknown JSON fields are ignored, so OKX additions are non-breaking.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Instrument {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID, e.g. `BTC-USDT`.
    pub inst_id: String,
    /// Underlying, e.g. `BTC-USD` (derivatives only).
    #[serde(default)]
    pub uly: String,
    /// Instrument family, e.g. `BTC-USD` (derivatives only).
    #[serde(default)]
    pub inst_family: String,
    /// Base currency, e.g. `BTC` (spot/margin only).
    #[serde(default)]
    pub base_ccy: String,
    /// Quote currency, e.g. `USDT` (spot/margin only).
    #[serde(default)]
    pub quote_ccy: String,
    /// Settlement currency (derivatives only).
    #[serde(default)]
    pub settle_ccy: String,
    /// Lot size (order size increment).
    #[serde(default)]
    pub lot_sz: NumberString,
    /// Tick size (price increment).
    #[serde(default)]
    pub tick_sz: NumberString,
    /// Minimum order size.
    #[serde(default)]
    pub min_sz: NumberString,
    /// Instrument lifecycle state, e.g. `live`, `suspend`.
    #[serde(default)]
    pub state: String,
}

/// OKX system time.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SystemTime {
    /// Current OKX system timestamp in Unix milliseconds.
    pub ts: NumberString,
}

/// Open interest for an instrument.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct OpenInterest {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID.
    pub inst_id: String,
    /// Open interest in contracts.
    #[serde(default)]
    pub oi: NumberString,
    /// Open interest in coin/currency units.
    #[serde(default)]
    pub oi_ccy: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Current funding-rate information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FundingRate {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    pub inst_id: String,
    /// Current funding rate.
    #[serde(default)]
    pub funding_rate: NumberString,
    /// Next estimated funding rate.
    #[serde(default)]
    pub next_funding_rate: NumberString,
    /// Funding time (Unix milliseconds).
    #[serde(default)]
    pub funding_time: NumberString,
    /// Next funding time (Unix milliseconds).
    #[serde(default)]
    pub next_funding_time: NumberString,
}

/// Historical funding-rate row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct FundingRateHistory {
    /// Instrument ID.
    pub inst_id: String,
    /// Funding rate.
    #[serde(default)]
    pub funding_rate: NumberString,
    /// Realized funding rate.
    #[serde(default)]
    pub realized_rate: NumberString,
    /// Funding time (Unix milliseconds).
    #[serde(default)]
    pub funding_time: NumberString,
    /// Funding method.
    #[serde(default)]
    pub method: String,
}

/// Price-limit information for an instrument.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PriceLimit {
    /// Instrument ID.
    pub inst_id: String,
    /// Highest buy price.
    #[serde(default)]
    pub buy_lmt: NumberString,
    /// Lowest sell price.
    #[serde(default)]
    pub sell_lmt: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Mark-price information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct MarkPrice {
    /// Instrument type.
    pub inst_type: InstType,
    /// Instrument ID.
    pub inst_id: String,
    /// Mark price.
    #[serde(default)]
    pub mark_px: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Delivery/exercise history row.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeliveryExercise {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Delivery/exercise price.
    #[serde(default)]
    pub px: NumberString,
    /// Delivery/exercise type.
    #[serde(rename = "type", default)]
    pub exercise_type: String,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Public position-tier information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PositionTier {
    /// Instrument type.
    pub inst_type: InstType,
    /// Trade mode.
    #[serde(default)]
    pub td_mode: String,
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Tier.
    #[serde(default)]
    pub tier: String,
    /// Minimum size.
    #[serde(default)]
    pub min_sz: NumberString,
    /// Maximum size.
    #[serde(default)]
    pub max_sz: NumberString,
    /// Initial margin rate.
    #[serde(default)]
    pub imr: NumberString,
    /// Maintenance margin rate.
    #[serde(default)]
    pub mmr: NumberString,
}

/// Insurance-fund snapshot.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InsuranceFund {
    /// Instrument type.
    #[serde(default)]
    pub inst_type: String,
    /// Fund type.
    #[serde(rename = "type", default)]
    pub fund_type: String,
    /// Currency.
    #[serde(default)]
    pub ccy: String,
    /// Balance amount.
    #[serde(default)]
    pub amt: NumberString,
    /// Timestamp (Unix milliseconds).
    #[serde(default)]
    pub ts: NumberString,
}

/// Contract/coin conversion result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ConvertContractCoin {
    /// Instrument ID.
    #[serde(default)]
    pub inst_id: String,
    /// Converted size.
    #[serde(default)]
    pub sz: NumberString,
    /// Conversion price.
    #[serde(default)]
    pub px: NumberString,
    /// Conversion unit.
    #[serde(default)]
    pub unit: String,
}

#[cfg(test)]
mod tests {
    use crate::OkxClient;
    use crate::model::InstType;
    use crate::test_util::MockTransport;

    #[tokio::test]
    async fn get_instruments_builds_request_and_parses() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SPOT","instId":"BTC-USDT","uly":"","instFamily":"",
             "baseCcy":"BTC","quoteCcy":"USDT","settleCcy":"","lotSz":"0.00000001",
             "tickSz":"0.1","minSz":"0.00001","state":"live"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let instruments = client
            .public_data()
            .get_instruments(InstType::Spot, None)
            .await
            .unwrap();

        assert_eq!(instruments.len(), 1);
        assert_eq!(instruments[0].inst_id, "BTC-USDT");
        assert_eq!(instruments[0].base_ccy, "BTC");
        assert_eq!(instruments[0].tick_sz.as_str(), "0.1");

        let req = mock.captured();
        assert_eq!(req.method, http::Method::GET);
        assert!(req.uri.ends_with("/api/v5/public/instruments?instType=SPOT"));
        assert!(!req.is_signed(), "public endpoint must not be signed");
    }

    #[tokio::test]
    async fn get_system_time_parses_time() {
        let body = r#"{"code":"0","msg":"","data":[{"ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let time = client.public_data().get_system_time().await.unwrap();
        assert_eq!(time[0].ts.as_str(), "1597026383085");

        let req = mock.captured();
        assert!(req.uri.ends_with("/api/v5/public/time"));
        assert_eq!(req.query(), None);
        assert!(!req.is_signed());
    }

    #[tokio::test]
    async fn get_open_interest_uses_family_request() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","instId":"BTC-USDT-SWAP","oi":"10","oiCcy":"1","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();
        let request = super::InstrumentFamilyRequest::new(InstType::Swap).inst_id("BTC-USDT-SWAP");

        let rows = client
            .public_data()
            .get_open_interest(&request)
            .await
            .unwrap();
        assert_eq!(rows[0].oi.as_str(), "10");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SWAP&instId=BTC-USDT-SWAP"));
    }

    #[tokio::test]
    async fn get_funding_rate_queries_instrument() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","fundingRate":"0.0001","nextFundingRate":"0.0002",
             "fundingTime":"1597026383085","nextFundingTime":"1597030000000"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let rows = client
            .public_data()
            .get_funding_rate("BTC-USDT-SWAP")
            .await
            .unwrap();
        assert_eq!(rows[0].funding_rate.as_str(), "0.0001");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instId=BTC-USDT-SWAP"));
    }

    #[tokio::test]
    async fn get_funding_rate_history_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","fundingRate":"0.0001","realizedRate":"0.0001",
             "fundingTime":"1597026383085","method":"current_period"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();
        let request = super::FundingRateHistoryRequest::new("BTC-USDT-SWAP")
            .before("10")
            .limit(1);

        let rows = client
            .public_data()
            .get_funding_rate_history(&request)
            .await
            .unwrap();
        assert_eq!(rows[0].method, "current_period");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instId=BTC-USDT-SWAP&before=10&limit=1"));
        assert!(!req.query().unwrap().contains("after"));
    }

    #[tokio::test]
    async fn get_price_limit_queries_instrument() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","buyLmt":"45000","sellLmt":"39000","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let rows = client
            .public_data()
            .get_price_limit("BTC-USDT-SWAP")
            .await
            .unwrap();
        assert_eq!(rows[0].buy_lmt.as_str(), "45000");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instId=BTC-USDT-SWAP"));
    }

    #[tokio::test]
    async fn get_mark_price_uses_family_request() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","instId":"BTC-USDT-SWAP","markPx":"42000","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();
        let request = super::InstrumentFamilyRequest::new(InstType::Swap).inst_family("BTC-USDT");

        let rows = client.public_data().get_mark_price(&request).await.unwrap();
        assert_eq!(rows[0].mark_px.as_str(), "42000");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SWAP&instFamily=BTC-USDT"));
    }

    #[tokio::test]
    async fn get_delivery_exercise_history_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"FUTURES","instId":"BTC-USD-240628","px":"42000","type":"delivery","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();
        let request = super::DeliveryExerciseHistoryRequest::new(InstType::Futures)
            .underlying("BTC-USD")
            .limit(1);

        let rows = client
            .public_data()
            .get_delivery_exercise_history(&request)
            .await
            .unwrap();
        assert_eq!(rows[0].exercise_type, "delivery");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=FUTURES&uly=BTC-USD&limit=1"));
    }

    #[tokio::test]
    async fn get_position_tiers_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","tdMode":"cross","instId":"BTC-USDT-SWAP","tier":"1",
             "minSz":"0","maxSz":"100","imr":"0.1","mmr":"0.05"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();
        let request = super::PositionTiersRequest::new(InstType::Swap, "cross").tier("1");

        let rows = client
            .public_data()
            .get_position_tiers(&request)
            .await
            .unwrap();
        assert_eq!(rows[0].tier, "1");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SWAP&tdMode=cross&tier=1"));
    }

    #[tokio::test]
    async fn get_underlying_queries_inst_type() {
        let body = r#"{"code":"0","msg":"","data":["BTC-USD"]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();

        let rows = client.public_data().get_underlying(InstType::Swap).await.unwrap();
        assert_eq!(rows[0], "BTC-USD");

        let req = mock.captured();
        assert_eq!(req.query(), Some("instType=SWAP"));
    }

    #[tokio::test]
    async fn get_insurance_fund_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instType":"SWAP","type":"regular_update","ccy":"USDT","amt":"100","ts":"1597026383085"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();
        let request = super::InsuranceFundRequest::new(InstType::Swap)
            .fund_type("regular_update")
            .currency("USDT");

        let rows = client
            .public_data()
            .get_insurance_fund(&request)
            .await
            .unwrap();
        assert_eq!(rows[0].amt.as_str(), "100");

        let req = mock.captured();
        assert_eq!(
            req.query(),
            Some("instType=SWAP&type=regular_update&ccy=USDT")
        );
    }

    #[tokio::test]
    async fn get_convert_contract_coin_uses_builder_query() {
        let body = r#"{"code":"0","msg":"","data":[
            {"instId":"BTC-USDT-SWAP","sz":"1","px":"42000","unit":"coin"}]}"#;
        let mock = MockTransport::new(body);
        let client = OkxClient::with_transport(mock.clone()).build();
        let request = super::ConvertContractCoinRequest::new("1", "BTC-USDT-SWAP", "1")
            .price("42000")
            .unit("coin");

        let rows = client
            .public_data()
            .get_convert_contract_coin(&request)
            .await
            .unwrap();
        assert_eq!(rows[0].unit, "coin");

        let req = mock.captured();
        assert_eq!(
            req.query(),
            Some("type=1&instId=BTC-USDT-SWAP&sz=1&px=42000&unit=coin")
        );
    }
}
