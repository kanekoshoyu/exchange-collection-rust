#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AuctionPayload represents a AuctionPayload model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AuctionPayload {
    #[serde(rename="type")]
    pub reserved_type: String,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="sequence")]
    pub sequence: i32,
    #[serde(rename="auction_state")]
    pub auction_state: String,
    #[serde(rename="best_bid_price")]
    pub best_bid_price: String,
    #[serde(rename="best_bid_size")]
    pub best_bid_size: String,
    #[serde(rename="best_ask_price")]
    pub best_ask_price: String,
    #[serde(rename="best_ask_size")]
    pub best_ask_size: String,
    #[serde(rename="open_price")]
    pub open_price: String,
    #[serde(rename="open_size")]
    pub open_size: String,
    #[serde(rename="can_open")]
    pub can_open: String,
    #[serde(rename="timestamp")]
    pub timestamp: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

