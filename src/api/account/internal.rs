use serde::Serialize;

use crate::model::InstType;

#[derive(Serialize)]
pub(super) struct NoQuery;

#[derive(Serialize)]
pub(super) struct EmptyBody {}

#[derive(Serialize)]
pub(super) struct BalanceQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) ccy: Option<&'a str>,
}

#[derive(Serialize)]
pub(super) struct PositionsQuery<'a> {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub(super) inst_type: Option<&'a InstType>,
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub(super) inst_id: Option<&'a str>,
}

#[derive(Serialize)]
pub(super) struct PositionRiskQuery<'a> {
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub(super) inst_type: Option<&'a InstType>,
}

#[derive(Serialize)]
pub(super) struct SetPositionModeBody<'a> {
    #[serde(rename = "posMode")]
    pub(super) pos_mode: &'a str,
}

#[derive(Serialize)]
pub(super) struct SetGreeksBody<'a> {
    #[serde(rename = "greeksType")]
    pub(super) greeks_type: &'a str,
}

#[derive(Serialize)]
pub(super) struct SetIsolatedModeBody<'a> {
    #[serde(rename = "isoMode")]
    pub(super) iso_mode: &'a str,
    #[serde(rename = "type")]
    pub(super) mode_type: &'a str,
}

#[derive(Serialize)]
pub(super) struct SetAutoLoanBody {
    #[serde(rename = "autoLoan")]
    pub(super) auto_loan: bool,
}

#[derive(Serialize)]
pub(super) struct SetAccountLevelBody<'a> {
    #[serde(rename = "acctLv")]
    pub(super) acct_lv: &'a str,
}
