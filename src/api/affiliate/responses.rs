use serde::Deserialize;

use crate::model::NumberString;

/// Performance for one affiliate commission category.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AffiliatePerformanceDetail {
    /// Commission category.
    #[serde(default)]
    pub commission_category: String,
    /// First-time trader count.
    #[serde(default)]
    pub first_trader_cnt: NumberString,
    /// Trader count.
    #[serde(default)]
    pub trader_cnt: NumberString,
    /// Trading volume.
    #[serde(default)]
    pub vol: NumberString,
    /// Commission amount.
    #[serde(default)]
    pub commission: NumberString,
}

/// Aggregate affiliate performance summary.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AffiliatePerformanceSummary {
    /// Last update timestamp.
    #[serde(default)]
    pub u_time: NumberString,
    /// Invitee count.
    #[serde(default)]
    pub invitee_cnt: NumberString,
    /// Deposit amount.
    #[serde(default)]
    pub dep_amt: NumberString,
    /// Performance by commission category.
    #[serde(default)]
    pub details: Vec<AffiliatePerformanceDetail>,
}

/// Detailed affiliate metrics for one invitee.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct InviteeDetail {
    /// Invitee level relative to the affiliate.
    #[serde(default)]
    pub invitee_level: String,
    /// Rebate relationship creation timestamp.
    #[serde(default)]
    pub join_time: NumberString,
    /// Invitee self-rebate rate.
    #[serde(default)]
    pub invitee_rebate_rate: NumberString,
    /// Total commission earned from the invitee.
    #[serde(default)]
    pub total_commission: NumberString,
    /// First trade timestamp.
    #[serde(default)]
    pub first_trade_time: NumberString,
    /// Invitee trading fee level.
    #[serde(default)]
    pub level: String,
    /// Accumulated deposit amount.
    #[serde(default)]
    pub dep_amt: NumberString,
    /// Accumulated withdrawal amount.
    #[serde(default)]
    pub wd_amt: NumberString,
    /// Current-month trading volume.
    #[serde(default)]
    pub vol_month: NumberString,
    /// Lifetime trading volume.
    #[serde(default)]
    pub total_vol: NumberString,
    /// Accumulated trading fee.
    #[serde(default)]
    pub acc_fee: NumberString,
    /// KYC2 verification timestamp.
    #[serde(default)]
    pub kyc_time: NumberString,
    /// User country or region.
    #[serde(default)]
    pub region: String,
    /// Affiliate invite code used by the invitee.
    #[serde(default)]
    pub affiliate_code: String,
}

/// Invitee row in the affiliate list.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Invitee {
    /// Invitee user ID.
    #[serde(default)]
    pub uid: String,
    /// Country code.
    #[serde(default)]
    pub country: String,
    /// Join timestamp.
    #[serde(default)]
    pub join_time: NumberString,
    /// First trade timestamp.
    #[serde(default)]
    pub first_trade_time: NumberString,
    /// Invitation channel name.
    #[serde(default)]
    pub channel_name: String,
    /// Rebate rate.
    #[serde(default)]
    pub rebate_rate: NumberString,
    /// Fee tier rank.
    #[serde(default)]
    pub fee_tier_rank: NumberString,
    /// KYC status.
    #[serde(default)]
    pub kyc_status: String,
    /// KYC timestamp.
    #[serde(default)]
    pub kyc_time: NumberString,
    /// Deposit amount.
    #[serde(default)]
    pub dep_amt: NumberString,
    /// Total trading volume.
    #[serde(default)]
    pub total_vol: NumberString,
    /// Total trading fee.
    #[serde(default)]
    pub total_fee: NumberString,
    /// Total commission.
    #[serde(default)]
    pub total_commission: NumberString,
    /// Whether the invitee is compliant.
    #[serde(default)]
    pub is_compliant: bool,
}

/// Standard affiliate invitation link.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AffiliateLink {
    /// Channel ID.
    #[serde(default)]
    pub channel_id: String,
    /// Channel name.
    #[serde(default)]
    pub channel_name: String,
    /// Invitation URL.
    #[serde(default)]
    pub join_link: String,
    /// Link type.
    #[serde(default)]
    pub link_type: String,
    /// Inviter commission rate.
    #[serde(default)]
    pub inviter_commission_rate: NumberString,
    /// Co-inviter commission rate.
    #[serde(default)]
    pub co_inviter_commission_rate: NumberString,
    /// Invitee discount rate.
    #[serde(default)]
    pub invitee_discount_rate: NumberString,
    /// Invitee count.
    #[serde(default)]
    pub invitee_cnt: NumberString,
    /// Trader count.
    #[serde(default)]
    pub trader_cnt: NumberString,
    /// Total commission.
    #[serde(default)]
    pub total_commission: NumberString,
    /// Commission earned in the last 24 hours.
    #[serde(default, rename = "commission24h")]
    pub commission_24h: NumberString,
    /// Creation timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// Whether this is the default link.
    #[serde(default)]
    pub is_default: bool,
    /// Link status.
    #[serde(default)]
    pub link_status: String,
}

/// Co-inviter invitation link.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CoInviterLink {
    /// Channel ID.
    #[serde(default)]
    pub channel_id: String,
    /// Channel name.
    #[serde(default)]
    pub channel_name: String,
    /// Invitation URL.
    #[serde(default)]
    pub join_link: String,
    /// Inviter commission rate.
    #[serde(default)]
    pub inviter_commission_rate: NumberString,
    /// Co-inviter commission rate.
    #[serde(default)]
    pub co_inviter_commission_rate: NumberString,
    /// Invitee discount rate.
    #[serde(default)]
    pub invitee_discount_rate: NumberString,
    /// Parent inviter display name.
    #[serde(default)]
    pub par_user_name: String,
    /// Co-inviter display name.
    #[serde(default)]
    pub co_user_name: String,
    /// Whether the link is compliant.
    #[serde(default)]
    pub is_compliant: bool,
    /// Link note.
    #[serde(default)]
    pub note: String,
    /// Whether this is the default link.
    #[serde(default)]
    pub is_default: bool,
    /// Total commission.
    #[serde(default)]
    pub total_commission: NumberString,
    /// Commission earned in the last 24 hours.
    #[serde(default, rename = "commission24h")]
    pub commission_24h: NumberString,
    /// Invitee count.
    #[serde(default)]
    pub invitee_cnt: NumberString,
    /// Trader count.
    #[serde(default)]
    pub trader_cnt: NumberString,
    /// Click count.
    #[serde(default)]
    pub click_cnt: NumberString,
    /// Total trading fee.
    #[serde(default)]
    pub total_fee: NumberString,
    /// Creation timestamp.
    #[serde(default)]
    pub c_time: NumberString,
    /// Channel assessment status.
    #[serde(default)]
    pub channel_assessment_status: String,
    /// Inviter channel status.
    #[serde(default)]
    pub inviter_channel_status: String,
    /// Co-inviter channel status.
    #[serde(default)]
    pub co_inviter_channel_status: String,
    /// Link status.
    #[serde(default)]
    pub link_status: String,
}

/// Sub-affiliate metrics.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SubAffiliate {
    /// Sub-affiliate user ID.
    #[serde(default)]
    pub sub_affiliate_uid: String,
    /// Country code.
    #[serde(default)]
    pub country: String,
    /// Join timestamp.
    #[serde(default)]
    pub join_time: NumberString,
    /// Sub-affiliate level.
    #[serde(default)]
    pub sub_affiliate_level: String,
    /// Commission rate.
    #[serde(default)]
    pub commission_rate: NumberString,
    /// Whether the sub-affiliate is compliant.
    #[serde(default)]
    pub is_compliant: bool,
    /// Invitee count.
    #[serde(default)]
    pub invitee_cnt: NumberString,
    /// Trader count.
    #[serde(default)]
    pub trader_cnt: NumberString,
    /// Deposit amount.
    #[serde(default)]
    pub dep_amt: NumberString,
    /// Total trading volume.
    #[serde(default)]
    pub total_vol: NumberString,
    /// Total trading fee.
    #[serde(default)]
    pub total_fee: NumberString,
    /// Total commission.
    #[serde(default)]
    pub total_commission: NumberString,
}
