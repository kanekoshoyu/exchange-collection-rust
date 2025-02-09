#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TickersArray represents a TickersArray model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TickersArray {
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub reserved_type: Option<String>,
    #[serde(rename="product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename="price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename="volume_24_h", skip_serializing_if = "Option::is_none")]
    pub volume_24_h: Option<String>,
    #[serde(rename="low_24_h", skip_serializing_if = "Option::is_none")]
    pub low_24_h: Option<String>,
    #[serde(rename="high_24_h", skip_serializing_if = "Option::is_none")]
    pub high_24_h: Option<String>,
    #[serde(rename="low_52_w", skip_serializing_if = "Option::is_none")]
    pub low_52_w: Option<String>,
    #[serde(rename="high_52_w", skip_serializing_if = "Option::is_none")]
    pub high_52_w: Option<String>,
    #[serde(rename="price_percent_chg_24_h", skip_serializing_if = "Option::is_none")]
    pub price_percent_chg_24_h: Option<String>,
    #[serde(rename="best_bid", skip_serializing_if = "Option::is_none")]
    pub best_bid: Option<String>,
    #[serde(rename="best_bid_quantity", skip_serializing_if = "Option::is_none")]
    pub best_bid_quantity: Option<String>,
    #[serde(rename="best_ask", skip_serializing_if = "Option::is_none")]
    pub best_ask: Option<String>,
    #[serde(rename="best_ask_quantity", skip_serializing_if = "Option::is_none")]
    pub best_ask_quantity: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

