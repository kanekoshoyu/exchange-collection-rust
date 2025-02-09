#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// RfqMatchesPayload represents a RfqMatchesPayload model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RfqMatchesPayload {
    #[serde(rename="type")]
    pub reserved_type: String,
    #[serde(rename="maker_order_id")]
    pub maker_order_id: String,
    #[serde(rename="taker_order_id")]
    pub taker_order_id: String,
    #[serde(rename="time")]
    pub time: String,
    #[serde(rename="trade_id")]
    pub trade_id: i32,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="size")]
    pub size: String,
    #[serde(rename="price")]
    pub price: String,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

