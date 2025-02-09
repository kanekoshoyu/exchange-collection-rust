#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OpenOrderMessage represents a OpenOrderMessage model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OpenOrderMessage {
    #[serde(rename="type")]
    pub reserved_type: String,
    #[serde(rename="time")]
    pub time: String,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="sequence")]
    pub sequence: i32,
    #[serde(rename="order_id")]
    pub order_id: String,
    #[serde(rename="price")]
    pub price: String,
    #[serde(rename="remaining_size")]
    pub remaining_size: String,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

