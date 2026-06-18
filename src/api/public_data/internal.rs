use serde::Serialize;

use crate::model::InstType;

#[derive(Serialize)]
pub(super) struct NoQuery;

#[derive(Serialize)]
pub(super) struct InstrumentsQuery<'a> {
    #[serde(rename = "instType")]
    pub(super) inst_type: &'a InstType,
    #[serde(rename = "instFamily", skip_serializing_if = "Option::is_none")]
    pub(super) inst_family: Option<&'a str>,
}

#[derive(Serialize)]
pub(super) struct InstIdQuery<'a> {
    #[serde(rename = "instId")]
    pub(super) inst_id: &'a str,
}

#[derive(Serialize)]
pub(super) struct CurrencyQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) ccy: Option<&'a str>,
}
