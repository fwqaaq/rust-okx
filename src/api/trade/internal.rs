use serde::Serialize;

#[derive(Serialize)]
pub(super) struct NoQuery;

#[derive(Serialize)]
pub(super) struct CancelOrderBody<'a> {
    #[serde(rename = "instId")]
    pub(super) inst_id: &'a str,
    #[serde(rename = "ordId")]
    pub(super) ord_id: &'a str,
}

#[derive(Serialize)]
pub(super) struct GetOrderQuery<'a> {
    #[serde(rename = "instId")]
    pub(super) inst_id: &'a str,
    #[serde(rename = "ordId")]
    pub(super) ord_id: &'a str,
}
