use crate::model::RequestParams;

pub(super) fn optional_ccy(ccy: Option<&str>) -> RequestParams {
    match ccy {
        Some(value) => RequestParams::new().param("ccy", value),
        None => RequestParams::new(),
    }
}
