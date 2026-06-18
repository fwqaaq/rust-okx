use serde::Serialize;

#[derive(Debug, Serialize)]
pub(super) struct CcyQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) ccy: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub(super) struct RequiredCcyQuery<'a> {
    pub(super) ccy: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TransferStateQuery<'a> {
    #[serde(rename = "transId")]
    pub(super) trans_id: &'a str,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub(super) transfer_type: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct WithdrawalIdBody<'a> {
    #[serde(rename = "wdId")]
    pub(super) wd_id: &'a str,
}
