#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// HoldingWalletResponse represents a HoldingWalletResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HoldingWalletResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="account")]
    pub account: String,
    #[serde(rename="holding")]
    pub holding: std::collections::HashMap<String, f64>,
    #[serde(rename="futures", skip_serializing_if = "Option::is_none")]
    pub futures: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename="timestamp")]
    pub timestamp: i64,
    #[serde(rename="seq")]
    pub seq: i64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

