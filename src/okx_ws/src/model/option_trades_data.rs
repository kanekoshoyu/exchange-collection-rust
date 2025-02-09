#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OptionTradesData represents a OptionTradesData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OptionTradesData {
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(rename="tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename="px", skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    #[serde(rename="sz", skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="optType", skip_serializing_if = "Option::is_none")]
    pub opt_type: Option<String>,
    #[serde(rename="fillVol", skip_serializing_if = "Option::is_none")]
    pub fill_vol: Option<String>,
    #[serde(rename="fwdPx", skip_serializing_if = "Option::is_none")]
    pub fwd_px: Option<String>,
    #[serde(rename="idxPx", skip_serializing_if = "Option::is_none")]
    pub idx_px: Option<String>,
    #[serde(rename="markPx", skip_serializing_if = "Option::is_none")]
    pub mark_px: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

