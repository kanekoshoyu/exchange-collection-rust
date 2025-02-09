#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PriceLimitData represents a PriceLimitData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PriceLimitData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="buyLmt", skip_serializing_if = "Option::is_none")]
    pub buy_lmt: Option<String>,
    #[serde(rename="sellLmt", skip_serializing_if = "Option::is_none")]
    pub sell_lmt: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

