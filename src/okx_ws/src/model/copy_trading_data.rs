#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CopyTradingData represents a CopyTradingData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CopyTradingData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="infoType", skip_serializing_if = "Option::is_none")]
    pub info_type: Option<String>,
    #[serde(rename="subPosId", skip_serializing_if = "Option::is_none")]
    pub sub_pos_id: Option<String>,
    #[serde(rename="uniqueCode", skip_serializing_if = "Option::is_none")]
    pub unique_code: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="lever", skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    #[serde(rename="avgPx", skip_serializing_if = "Option::is_none")]
    pub avg_px: Option<String>,
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
    #[serde(rename="slTotalAmt", skip_serializing_if = "Option::is_none")]
    pub sl_total_amt: Option<String>,
    #[serde(rename="rmThold", skip_serializing_if = "Option::is_none")]
    pub rm_thold: Option<String>,
    #[serde(rename="minNotional", skip_serializing_if = "Option::is_none")]
    pub min_notional: Option<String>,
    #[serde(rename="copyTotalAmt", skip_serializing_if = "Option::is_none")]
    pub copy_total_amt: Option<String>,
    #[serde(rename="slippageRatio", skip_serializing_if = "Option::is_none")]
    pub slippage_ratio: Option<String>,
    #[serde(rename="maxLeadTraderNum", skip_serializing_if = "Option::is_none")]
    pub max_lead_trader_num: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

