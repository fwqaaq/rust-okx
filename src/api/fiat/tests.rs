use http::Method;

use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

use super::{
    CreateFiatWithdrawalRequest, FiatBuySellHistoryRequest, FiatBuySellPairRequest,
    FiatBuySellQuoteRequest, FiatBuySellSide, FiatBuySellTradeRequest, FiatCurrencyRequest,
    FiatOrderHistoryRequest, FiatOrderIdRequest,
};

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[tokio::test]
async fn payment_methods_match_official_nested_response() {
    let body = r#"{"code":"0","msg":"","data":[{"ccy":"TRY","paymentMethod":"TR_BANKS","feeRate":"0","minFee":"0","limits":{"dailyLimit":"2147483647","dailyLimitRemaining":"2147483647","weeklyLimit":"2147483647","weeklyLimitRemaining":"2147483647","monthlyLimit":"","monthlyLimitRemaining":"","maxAmt":"1000000","minAmt":"1","lifetimeLimit":"2147483647"},"accounts":[{"paymentAcctId":"1","acctNum":"TR740001592093703829602611","recipientName":"John Doe","bankName":"VakıfBank","bankCode":"TVBATR2AXXX","state":"active"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = FiatCurrencyRequest::new("TRY");

    let rows = client
        .fiat()
        .get_deposit_payment_methods(&request)
        .await
        .unwrap();
    assert_eq!(rows[0].limits.min_amt.as_str(), "1");
    assert_eq!(rows[0].accounts[0].payment_acct_id, "1");
    assert_eq!(mock.captured().query(), Some("ccy=TRY"));
    assert!(mock.captured().is_signed());

    client
        .fiat()
        .get_withdrawal_payment_methods(&request)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .contains("/api/v5/fiat/withdrawal-payment-methods?")
    );
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn withdrawal_and_deposit_order_endpoints_match_official_examples() {
    let order_body = r#"{"code":"0","msg":"","data":[{"cTime":"1707429385000","uTime":"1707429385000","ordId":"124041201450544699","paymentMethod":"TR_BANKS","paymentAcctId":"20","fee":"0","amt":"100","ccy":"TRY","state":"completed","clientId":"194a6975e98246538faeb0fab0d502df"}]}"#;
    let mock = MockTransport::new(order_body);
    let client = signed_client(mock.clone());
    let create = CreateFiatWithdrawalRequest::new(
        "412323",
        "TRY",
        "10000",
        "TR_BANKS",
        "194a6975e98246538faeb0fab0d502df",
    );

    let rows = client.fiat().create_withdrawal(&create).await.unwrap();
    assert_eq!(rows[0].ord_id, "124041201450544699");
    assert_eq!(rows[0].amt.as_str(), "100");
    assert_eq!(
        mock.captured().body_str(),
        r#"{"paymentAcctId":"412323","ccy":"TRY","amt":"10000","paymentMethod":"TR_BANKS","clientId":"194a6975e98246538faeb0fab0d502df"}"#
    );

    let history = FiatOrderHistoryRequest::new()
        .currency("TRY")
        .payment_method("TR_BANKS")
        .state("completed")
        .after("1597026383085")
        .before("1707429385000")
        .limit(100);
    client
        .fiat()
        .get_withdrawal_order_history(&history)
        .await
        .unwrap();
    assert_eq!(
        mock.captured().query(),
        Some(
            "ccy=TRY&paymentMethod=TR_BANKS&state=completed&after=1597026383085&before=1707429385000&limit=100"
        )
    );
    client
        .fiat()
        .get_deposit_order_history(&history)
        .await
        .unwrap();
    assert!(
        mock.captured()
            .uri
            .contains("/api/v5/fiat/deposit-order-history?")
    );

    let id = FiatOrderIdRequest::new("024041201450544699");
    client.fiat().get_withdrawal(&id).await.unwrap();
    assert!(
        mock.captured()
            .uri
            .contains("/api/v5/fiat/withdrawal?ordId=024041201450544699")
    );
    client.fiat().get_deposit(&id).await.unwrap();
    assert!(
        mock.captured()
            .uri
            .contains("/api/v5/fiat/deposit?ordId=024041201450544699")
    );

    let cancel_body =
        r#"{"code":"0","msg":"","data":[{"ordId":"124041201450544699","state":"canceled"}]}"#;
    let mock = MockTransport::new(cancel_body);
    let client = signed_client(mock.clone());
    let rows = client
        .fiat()
        .cancel_withdrawal(&FiatOrderIdRequest::new("124041201450544699"))
        .await
        .unwrap();
    assert_eq!(rows[0].state, "canceled");
    assert_eq!(
        mock.captured().body_str(),
        r#"{"ordId":"124041201450544699"}"#
    );
    assert_eq!(mock.captured().method, Method::POST);
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn buy_sell_catalog_endpoints_match_official_examples() {
    let body = r#"{"code":"0","data":[{"fiatCcyList":[{"ccy":"USD"},{"ccy":"EUR"}],"cryptoCcyList":[{"ccy":"BTC"}]}],"msg":""}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let rows = client.fiat().get_buy_sell_currencies().await.unwrap();
    assert_eq!(rows[0].fiat_ccy_list[0].ccy, "USD");
    assert_eq!(rows[0].crypto_ccy_list[0].ccy, "BTC");
    assert_eq!(mock.captured().query(), None);
    assert!(mock.captured().is_signed());

    let pair_body = r#"{"code":"0","data":[{"side":"buy","fromCcy":"USD","toCcy":"BTC","singleTradeMax":"1","singleTradeMin":"0.01","fixedPxRemainingDailyQuota":"","fixedPxDailyLimit":"","paymentMethods":["balance"]}],"msg":""}"#;
    let mock = MockTransport::new(pair_body);
    let client = signed_client(mock.clone());
    let rows = client
        .fiat()
        .get_buy_sell_currency_pair(&FiatBuySellPairRequest::new("USD", "BTC"))
        .await
        .unwrap();
    assert_eq!(rows[0].single_trade_min.as_str(), "0.01");
    assert_eq!(rows[0].payment_methods, ["balance"]);
    assert_eq!(mock.captured().query(), Some("fromCcy=USD&toCcy=BTC"));
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn buy_sell_quote_trade_and_history_match_official_examples() {
    let quote_body = r#"{"code":"0","data":[{"quoteId":"quoterBTC-USD16461885104612381","fromCcy":"USD","toCcy":"BTC","rfqAmt":"30","rfqCcy":"USD","quotePx":"2932.40104429","quoteCcy":"USD","quoteFromAmt":"30","quoteToAmt":"30","quoteTime":"1646188510461","ttlMs":"10000"}],"msg":""}"#;
    let mock = MockTransport::new(quote_body);
    let client = signed_client(mock.clone());
    let quote = FiatBuySellQuoteRequest::new(FiatBuySellSide::Buy, "USD", "BTC", "30", "USD");
    let rows = client.fiat().get_buy_sell_quote(&quote).await.unwrap();
    assert_eq!(rows[0].quote_px.as_str(), "2932.40104429");
    assert_eq!(
        mock.captured().body_str(),
        r#"{"side":"buy","fromCcy":"USD","toCcy":"BTC","rfqAmt":"30","rfqCcy":"USD"}"#
    );

    let trade_body = r#"{"code":"0","data":[{"ordId":"1234","clOrdId":"","quoteId":"quoterBTC-USD16461885104612381","state":"completed","side":"buy","fromCcy":"USD","toCcy":"BTC","rfqAmt":"30","rfqCcy":"USD","fillPx":"2932.40104429","fillQuoteCcy":"USD","fillFromAmt":"30","fillToAmt":"0.01","cTime":"1646188510461","uTime":"1646188510461"}],"msg":""}"#;
    let mock = MockTransport::new(trade_body);
    let client = signed_client(mock.clone());
    let trade = FiatBuySellTradeRequest::new(
        "quoterBTC-USD16461885104612381",
        FiatBuySellSide::Buy,
        "USD",
        "BTC",
        "30",
        "USD",
        "balance",
        "123456",
    );
    let rows = client.fiat().buy_sell_trade(&trade).await.unwrap();
    assert_eq!(rows[0].ord_id, "1234");
    assert_eq!(rows[0].fill_to_amt.as_str(), "0.01");

    let history = FiatBuySellHistoryRequest::new()
        .order_id("1234")
        .client_order_id("123456")
        .state("completed")
        .begin("1646188000000")
        .end("1646189000000")
        .limit(100);
    client.fiat().get_buy_sell_history(&history).await.unwrap();
    assert_eq!(
        mock.captured().query(),
        Some(
            "ordId=1234&clOrdId=123456&state=completed&begin=1646188000000&end=1646189000000&limit=100"
        )
    );
    assert!(mock.captured().is_signed());
}
