use http::Method;

use crate::test_util::MockTransport;

use super::super::{
    DualInvestmentHistoryRequest, DualInvestmentOptionType, DualInvestmentOrderIdRequest,
    DualInvestmentProductsRequest, DualInvestmentQuoteIdRequest, DualInvestmentQuoteRequest,
    DualInvestmentRedeemRequest,
};
use super::signed_client;

#[tokio::test]
async fn catalog_and_products_match_official_examples() {
    let pair_body = r#"{"code":"0","msg":"","data":[{"baseCcy":"BTC","quoteCcy":"USDT","optType":"C","uly":"BTC-USD"}]}"#;
    let mock = MockTransport::new(pair_body);
    let client = signed_client(mock.clone());
    let rows = client
        .finance()
        .dual_investment()
        .get_currency_pairs()
        .await
        .unwrap();
    assert_eq!(rows[0].base_ccy, "BTC");
    assert_eq!(rows[0].opt_type, "C");
    assert_eq!(mock.captured().query(), None);
    assert!(mock.captured().is_signed());

    let product_body = r#"{"code":"0","msg":"","data":[{"absYield":"0.00232413","annualizedYield":"0.0541","baseCcy":"BTC","quoteCcy":"USDT","expTime":"1774598400000","interestAccrualTime":"1773244800000","listTime":"1743150759000","maxSize":"6000000","minSize":"10","notionalCcy":"USDT","optType":"P","productId":"BTC-USDT-260327-54500-P","quoteTime":"1773243808703","redeemEndTime":"1774594800000","redeemStartTime":"1773244800000","stepSz":"1","tradeEndTime":"1774584000000","strike":"54500","uly":"BTC-USD"}]}"#;
    let mock = MockTransport::new(product_body);
    let client = signed_client(mock.clone());
    let request =
        DualInvestmentProductsRequest::new("BTC", "USDT", DualInvestmentOptionType::Call);
    let rows = client
        .finance()
        .dual_investment()
        .get_products(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].product_id, "BTC-USDT-260327-54500-P");
    assert_eq!(rows[0].strike.as_str(), "54500");
    assert_eq!(
        mock.captured().query(),
        Some("baseCcy=BTC&quoteCcy=USDT&optType=C")
    );
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn quote_and_trade_match_official_examples() {
    let quote_body = r#"{"code":"0","msg":"","data":[{"absYield":"0.00135182","annualizedYield":"69.65","interestAccrualTime":"1773241200000","notionalSz":"0.001","notionalCcy":"BTC","productId":"BTC-USDT-260312-72000-C","quoteId":"qtbcDCD-QUOTE17732395560537636","validUntil":"1774584000000","idxPx":"69000"}]}"#;
    let mock = MockTransport::new(quote_body);
    let client = signed_client(mock.clone());
    let request =
        DualInvestmentQuoteRequest::new("BTC-USDT-260327-77000-C", "1.5", "BTC");
    let rows = client
        .finance()
        .dual_investment()
        .request_quote(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].idx_px.as_str(), "69000");
    assert_eq!(
        mock.captured().body_str(),
        r#"{"productId":"BTC-USDT-260327-77000-C","notionalSz":"1.5","notionalCcy":"BTC"}"#
    );
    assert!(mock.captured().is_signed());

    let trade_body = r#"{"code":"0","msg":"","data":[{"quoteId":"quoterbpDCD-QUOTE17732116652401234","ordId":"987654321","state":"live"}]}"#;
    let mock = MockTransport::new(trade_body);
    let client = signed_client(mock.clone());
    let rows = client
        .finance()
        .dual_investment()
        .trade(&DualInvestmentQuoteIdRequest::new(
            "quoterbpDCD-QUOTE17732116652401234",
        ))
        .await
        .unwrap();
    assert_eq!(rows[0].ord_id, "987654321");
    assert_eq!(mock.captured().method, Method::POST);
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn redeem_flow_and_status_match_official_examples() {
    let quote_body = r#"{"code":"0","msg":"","data":[{"ordId":"987654321","quoteId":"quoterbcDCD-REDEEM17732116652401234","redeemCcy":"BTC","redeemSz":"1.4856","termRate":"-0.50","validUntil":"1774598400000"}]}"#;
    let mock = MockTransport::new(quote_body);
    let client = signed_client(mock.clone());
    let id = DualInvestmentOrderIdRequest::new("987654321");
    let rows = client
        .finance()
        .dual_investment()
        .request_redeem_quote(&id)
        .await
        .unwrap();
    assert_eq!(rows[0].redeem_sz.as_str(), "1.4856");
    assert_eq!(mock.captured().body_str(), r#"{"ordId":"987654321"}"#);

    let state_body = r#"{"code":"0","msg":"","data":[{"ordId":"987654321","state":"pending_redeem_booking"}]}"#;
    let mock = MockTransport::new(state_body);
    let client = signed_client(mock.clone());
    let rows = client
        .finance()
        .dual_investment()
        .redeem(&DualInvestmentRedeemRequest::new(
            "987654321",
            "quoterbcDCD-REDEEM17732116652401234",
        ))
        .await
        .unwrap();
    assert_eq!(rows[0].state, "pending_redeem_booking");
    client
        .finance()
        .dual_investment()
        .get_order_status(&id)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .contains("/api/v5/finance/sfp/dcd/order-status?")
    );
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn order_history_matches_official_example() {
    let body = r#"{"code":"0","msg":"","data":[{"ordId":"987654321","quoteId":"quoterbpDCD-QUOTE17732116652401234","state":"settled","productId":"BTC-USDT-260327-77000-C","baseCcy":"BTC","quoteCcy":"USDT","uly":"BTC-USD","strike":"77000","notionalSz":"1.5","notionalCcy":"BTC","absYield":"0.00806038","annualizedYield":"0.1834","yieldSz":"0.01209057","yieldCcy":"BTC","settleSz":"1.51209057","settleCcy":"BTC","settlePx":"76500","settleTime":"1774598400000","expTime":"1774598400000","redeemStartTime":"1773244800000","redeemEndTime":"1774594800000","cTime":"1773212400000","uTime":"1773212400000"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = DualInvestmentHistoryRequest::new()
        .order_id("987654321")
        .product_id("BTC-USDT-260327-77000-C")
        .underlying("BTC-USD")
        .state("settled")
        .begin_id("987654320")
        .end_id("987654322")
        .begin("1773210000000")
        .end("1774600000000")
        .limit(100);
    let rows = client
        .finance()
        .dual_investment()
        .get_order_history(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].settle_sz.as_str(), "1.51209057");
    assert_eq!(rows[0].redeem_end_time.as_str(), "1774594800000");
    assert_eq!(
        mock.captured().query(),
        Some(
            "ordId=987654321&productId=BTC-USDT-260327-77000-C&uly=BTC-USD&state=settled&beginId=987654320&endId=987654322&begin=1773210000000&end=1774600000000&limit=100"
        )
    );
    assert!(mock.captured().is_signed());
}
