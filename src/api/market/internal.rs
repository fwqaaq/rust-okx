use serde::Serialize;

use crate::model::InstType;

#[derive(Serialize)]
pub(super) struct NoQuery;

#[derive(Serialize)]
pub(super) struct InstIdQuery<'a> {
    #[serde(rename = "instId")]
    pub(super) inst_id: &'a str,
}

#[derive(Serialize)]
pub(super) struct TickersQuery<'a> {
    #[serde(rename = "instType")]
    pub(super) inst_type: &'a InstType,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub(super) inst_family: Option<&'a str>,
}

#[derive(Serialize)]
pub(super) struct IndexTickersQuery<'a> {
    #[serde(rename = "quoteCcy", skip_serializing_if = "Option::is_none")]
    pub(super) quote_ccy: Option<&'a str>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub(super) inst_id: Option<&'a str>,
}

#[derive(Serialize)]
pub(super) struct OrderBookQuery<'a> {
    #[serde(rename = "instId")]
    pub(super) inst_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) sz: Option<u32>,
}

#[derive(Serialize)]
pub(super) struct CandlesQuery<'a> {
    #[serde(rename = "instId")]
    pub(super) inst_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) bar: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) limit: Option<u32>,
}

#[derive(Serialize)]
pub(super) struct TradesQuery<'a> {
    #[serde(rename = "instId")]
    pub(super) inst_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) limit: Option<u32>,
}

#[derive(Serialize)]
pub(super) struct InstFamilyQuery<'a> {
    #[serde(rename = "instFamily")]
    pub(super) inst_family: &'a str,
}

#[derive(Serialize)]
pub(super) struct IndexComponentsQuery<'a> {
    pub(super) index: &'a str,
}
