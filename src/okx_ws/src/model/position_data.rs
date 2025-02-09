#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PositionData represents a PositionData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PositionData {
    #[serde(rename="posId", skip_serializing_if = "Option::is_none")]
    pub pos_id: Option<String>,
    #[serde(rename="tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="mgnMode", skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<String>,
    #[serde(rename="avgPx", skip_serializing_if = "Option::is_none")]
    pub avg_px: Option<String>,
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<Box<PosEnum>>,
    #[serde(rename="pos", skip_serializing_if = "Option::is_none")]
    pub pos: Option<String>,
    #[serde(rename="posCcy", skip_serializing_if = "Option::is_none")]
    pub pos_ccy: Option<String>,
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

