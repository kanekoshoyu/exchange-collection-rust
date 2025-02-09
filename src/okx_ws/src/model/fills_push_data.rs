#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FillsPushData represents a FillsPushData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FillsPushData {
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="fillSz", skip_serializing_if = "Option::is_none")]
    pub fill_sz: Option<String>,
    #[serde(rename="fillPx", skip_serializing_if = "Option::is_none")]
    pub fill_px: Option<String>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(rename="tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename="execType", skip_serializing_if = "Option::is_none")]
    pub exec_type: Option<String>,
    #[serde(rename="count", skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

