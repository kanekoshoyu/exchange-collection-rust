#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TickerMessage represents a TickerMessage model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TickerMessage {
    #[serde(rename="type")]
    pub reserved_type: String,
    #[serde(rename="sequence")]
    pub sequence: i32,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="price")]
    pub price: String,
    #[serde(rename="open_24h")]
    pub open_24h: String,
    #[serde(rename="volume_24h")]
    pub volume_24h: String,
    #[serde(rename="low_24h")]
    pub low_24h: String,
    #[serde(rename="high_24h")]
    pub high_24h: String,
    #[serde(rename="volume_30d")]
    pub volume_30d: String,
    #[serde(rename="best_bid")]
    pub best_bid: String,
    #[serde(rename="best_bid_size")]
    pub best_bid_size: String,
    #[serde(rename="best_ask")]
    pub best_ask: String,
    #[serde(rename="best_ask_size")]
    pub best_ask_size: String,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="time")]
    pub time: String,
    #[serde(rename="trade_id")]
    pub trade_id: i32,
    #[serde(rename="last_size", skip_serializing_if = "Option::is_none")]
    pub last_size: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

