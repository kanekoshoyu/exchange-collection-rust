#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeChannelPushData represents a TradeChannelPushData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeChannelPushData {
    #[serde(rename="sprdId", skip_serializing_if = "Option::is_none")]
    pub sprd_id: Option<String>,
    #[serde(rename="tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename="ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(rename="clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(rename="tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename="fillPx", skip_serializing_if = "Option::is_none")]
    pub fill_px: Option<String>,
    #[serde(rename="fillSz", skip_serializing_if = "Option::is_none")]
    pub fill_sz: Option<String>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename="execType", skip_serializing_if = "Option::is_none")]
    pub exec_type: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="legs", skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<Legs>>,
    #[serde(rename="code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename="msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

