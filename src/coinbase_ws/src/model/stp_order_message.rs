#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// StpOrderMessage represents a StpOrderMessage model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct StpOrderMessage {
    #[serde(rename="type")]
    pub reserved_type: String,
    #[serde(rename="reason")]
    pub reason: Box<StpEnum>,
    #[serde(rename="time")]
    pub time: String,
    #[serde(rename="sequence")]
    pub sequence: i32,
    #[serde(rename="order_id")]
    pub order_id: String,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="old_size")]
    pub old_size: String,
    #[serde(rename="new_size")]
    pub new_size: String,
    #[serde(rename="price")]
    pub price: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

