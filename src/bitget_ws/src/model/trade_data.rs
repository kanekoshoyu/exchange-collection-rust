#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeData represents a TradeData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeData {
    #[serde(rename="ts")]
    pub ts: String,
    #[serde(rename="price")]
    pub price: String,
    #[serde(rename="size")]
    pub size: String,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="tradeId")]
    pub trade_id: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

