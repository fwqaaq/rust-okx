use super::{InviteeDetailRequest, InviteeListRequest, PerformanceSummaryRequest};
use crate::test_util::MockTransport;
use crate::{Credentials, OkxClient};

fn signed_client(mock: MockTransport) -> OkxClient<MockTransport> {
    OkxClient::with_transport(mock)
        .credentials(Credentials::new("key", "secret", "pass"))
        .build()
}

#[tokio::test]
async fn performance_summary_preserves_official_precision() {
    let body = r#"{"code":"0","msg":"","data":[{"uTime":"1777541513000","inviteeCnt":"102","depAmt":"1756.287846940199989393","details":[{"commissionCategory":"SPOT","firstTraderCnt":"17","traderCnt":"17","vol":"21548.6417826825604","commission":"3.322319946747010328"}]}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .affiliate()
        .get_performance_summary(&PerformanceSummaryRequest {
            period_type: Some("total".into()),
            ..Default::default()
        })
        .await
        .unwrap();

    assert_eq!(rows[0].dep_amt.as_str(), "1756.287846940199989393");
    assert_eq!(
        rows[0].details[0].commission.as_str(),
        "3.322319946747010328"
    );
    assert_eq!(mock.captured().query(), Some("periodType=total"));
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn invitee_detail_matches_official_example() {
    let body = r#"{"msg":"","code":"0","data":[{"accFee":"0","affiliateCode":"HIIIIII","depAmt":"0","wdAmt":"0","firstTradeTime":"","inviteeLevel":"2","inviteeRebateRate":"0.39","joinTime":"1712546713000","kycTime":"","level":"Lv1","region":"Vietnam","totalCommission":"0","volMonth":"0","totalVol":"0"}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());

    let rows = client
        .affiliate()
        .get_invitee_detail(&InviteeDetailRequest {
            uid: "11111111".into(),
        })
        .await
        .unwrap();

    assert_eq!(rows[0].affiliate_code, "HIIIIII");
    assert_eq!(rows[0].invitee_rebate_rate.as_str(), "0.39");
    assert!(mock.captured().is_signed());
}

#[tokio::test]
async fn invitee_list_retains_total_page_metadata() {
    let body = r#"{"code":"0","msg":"","totalPage":"5","data":[{"uid":"835449167911924693","country":"CN","joinTime":"1777448564000","firstTradeTime":"","channelName":"X2UWA2T89","rebateRate":"0.1600","feeTierRank":"0","kycStatus":"verified","kycTime":"1777448563000","depAmt":"0.0000000000","totalVol":"0.0000000000","totalFee":"0.0000000000","totalCommission":"0.0000000000","isCompliant":true}]}"#;
    let mock = MockTransport::new(body);
    let client = signed_client(mock.clone());
    let request = InviteeListRequest {
        page: Some("1".into()),
        kyc_status: Some("verified".into()),
        ..Default::default()
    };

    let page = client.affiliate().get_invitees(&request).await.unwrap();

    assert_eq!(page.total_page.as_str(), "5");
    assert_eq!(page.data[0].uid, "835449167911924693");
    assert_eq!(mock.captured().query(), Some("page=1&kycStatus=verified"));
    assert!(mock.captured().is_signed());
}
