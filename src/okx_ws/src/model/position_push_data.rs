#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PositionPushData represents a PositionPushData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PositionPushData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="mgnMode", skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<String>,
    #[serde(rename="posId", skip_serializing_if = "Option::is_none")]
    pub pos_id: Option<String>,
    #[serde(rename="posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<Box<PosEnum>>,
    #[serde(rename="pos", skip_serializing_if = "Option::is_none")]
    pub pos: Option<String>,
    #[serde(rename="baseBal", skip_serializing_if = "Option::is_none")]
    pub base_bal: Option<String>,
    #[serde(rename="quoteBal", skip_serializing_if = "Option::is_none")]
    pub quote_bal: Option<String>,
    #[serde(rename="posCcy", skip_serializing_if = "Option::is_none")]
    pub pos_ccy: Option<String>,
    #[serde(rename="avgPx", skip_serializing_if = "Option::is_none")]
    pub avg_px: Option<String>,
    #[serde(rename="upl", skip_serializing_if = "Option::is_none")]
    pub upl: Option<String>,
    #[serde(rename="uplRatio", skip_serializing_if = "Option::is_none")]
    pub upl_ratio: Option<String>,
    #[serde(rename="markPx", skip_serializing_if = "Option::is_none")]
    pub mark_px: Option<String>,
    #[serde(rename="imr", skip_serializing_if = "Option::is_none")]
    pub imr: Option<String>,
    #[serde(rename="mgnRatio", skip_serializing_if = "Option::is_none")]
    pub mgn_ratio: Option<String>,
    #[serde(rename="liab", skip_serializing_if = "Option::is_none")]
    pub liab: Option<String>,
    #[serde(rename="realizedPnl", skip_serializing_if = "Option::is_none")]
    pub realized_pnl: Option<String>,
    #[serde(rename="closeOrderAlgo", skip_serializing_if = "Option::is_none")]
    pub close_order_algo: Option<Vec<CloseOrderAlgo>>,
    #[serde(rename="cTime", skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="pTime", skip_serializing_if = "Option::is_none")]
    pub p_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

