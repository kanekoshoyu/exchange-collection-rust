#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PushDetails represents a PushDetails model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PushDetails {
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="eq", skip_serializing_if = "Option::is_none")]
    pub eq: Option<String>,
    #[serde(rename="cashBal", skip_serializing_if = "Option::is_none")]
    pub cash_bal: Option<String>,
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="isoEq", skip_serializing_if = "Option::is_none")]
    pub iso_eq: Option<String>,
    #[serde(rename="availEq", skip_serializing_if = "Option::is_none")]
    pub avail_eq: Option<String>,
    #[serde(rename="disEq", skip_serializing_if = "Option::is_none")]
    pub dis_eq: Option<String>,
    #[serde(rename="fixedBal", skip_serializing_if = "Option::is_none")]
    pub fixed_bal: Option<String>,
    #[serde(rename="availBal", skip_serializing_if = "Option::is_none")]
    pub avail_bal: Option<String>,
    #[serde(rename="frozenBal", skip_serializing_if = "Option::is_none")]
    pub frozen_bal: Option<String>,
    #[serde(rename="ordFrozen", skip_serializing_if = "Option::is_none")]
    pub ord_frozen: Option<String>,
    #[serde(rename="liab", skip_serializing_if = "Option::is_none")]
    pub liab: Option<String>,
    #[serde(rename="upl", skip_serializing_if = "Option::is_none")]
    pub upl: Option<String>,
    #[serde(rename="uplLiab", skip_serializing_if = "Option::is_none")]
    pub upl_liab: Option<String>,
    #[serde(rename="crossLiab", skip_serializing_if = "Option::is_none")]
    pub cross_liab: Option<String>,
    #[serde(rename="isoLiab", skip_serializing_if = "Option::is_none")]
    pub iso_liab: Option<String>,
    #[serde(rename="rewardBal", skip_serializing_if = "Option::is_none")]
    pub reward_bal: Option<String>,
    #[serde(rename="mgnRatio", skip_serializing_if = "Option::is_none")]
    pub mgn_ratio: Option<String>,
    #[serde(rename="imr", skip_serializing_if = "Option::is_none")]
    pub imr: Option<String>,
    #[serde(rename="mmr", skip_serializing_if = "Option::is_none")]
    pub mmr: Option<String>,
    #[serde(rename="interest", skip_serializing_if = "Option::is_none")]
    pub interest: Option<String>,
    #[serde(rename="twap", skip_serializing_if = "Option::is_none")]
    pub twap: Option<String>,
    #[serde(rename="maxLoan", skip_serializing_if = "Option::is_none")]
    pub max_loan: Option<String>,
    #[serde(rename="eqUsd", skip_serializing_if = "Option::is_none")]
    pub eq_usd: Option<String>,
    #[serde(rename="borrowFroz", skip_serializing_if = "Option::is_none")]
    pub borrow_froz: Option<String>,
    #[serde(rename="notionalLever", skip_serializing_if = "Option::is_none")]
    pub notional_lever: Option<String>,
    #[serde(rename="coinUsdPrice", skip_serializing_if = "Option::is_none")]
    pub coin_usd_price: Option<String>,
    #[serde(rename="stgyEq", skip_serializing_if = "Option::is_none")]
    pub stgy_eq: Option<String>,
    #[serde(rename="isoUpl", skip_serializing_if = "Option::is_none")]
    pub iso_upl: Option<String>,
    #[serde(rename="spotInUseAmt", skip_serializing_if = "Option::is_none")]
    pub spot_in_use_amt: Option<String>,
    #[serde(rename="clSpotInUseAmt", skip_serializing_if = "Option::is_none")]
    pub cl_spot_in_use_amt: Option<String>,
    #[serde(rename="maxSpotInUseAmt", skip_serializing_if = "Option::is_none")]
    pub max_spot_in_use_amt: Option<String>,
    #[serde(rename="smtSyncEq", skip_serializing_if = "Option::is_none")]
    pub smt_sync_eq: Option<String>,
    #[serde(rename="spotCopyTradingEq", skip_serializing_if = "Option::is_none")]
    pub spot_copy_trading_eq: Option<String>,
    #[serde(rename="spotBal", skip_serializing_if = "Option::is_none")]
    pub spot_bal: Option<String>,
    #[serde(rename="openAvgPx", skip_serializing_if = "Option::is_none")]
    pub open_avg_px: Option<Vec<String>>,
    #[serde(rename="accAvgPx", skip_serializing_if = "Option::is_none")]
    pub acc_avg_px: Option<Vec<String>>,
    #[serde(rename="spotUpl", skip_serializing_if = "Option::is_none")]
    pub spot_upl: Option<String>,
    #[serde(rename="spotUplRatio", skip_serializing_if = "Option::is_none")]
    pub spot_upl_ratio: Option<String>,
    #[serde(rename="totalPnl", skip_serializing_if = "Option::is_none")]
    pub total_pnl: Option<String>,
    #[serde(rename="totalPnlRatio", skip_serializing_if = "Option::is_none")]
    pub total_pnl_ratio: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

