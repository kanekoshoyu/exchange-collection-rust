#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Legs represents a Legs model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Legs {
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="px", skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    #[serde(rename="sz", skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    #[serde(rename="szCont", skip_serializing_if = "Option::is_none")]
    pub sz_cont: Option<String>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="fillPnl", skip_serializing_if = "Option::is_none")]
    pub fill_pnl: Option<String>,
    #[serde(rename="fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    #[serde(rename="feeCcy", skip_serializing_if = "Option::is_none")]
    pub fee_ccy: Option<String>,
    #[serde(rename="tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

