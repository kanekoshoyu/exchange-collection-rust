#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// LimitOrderMessage represents a LimitOrderMessage model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LimitOrderMessage {
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
    #[serde(rename="size")]
    pub size: String,
    #[serde(rename="price")]
    pub price: String,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="order_type")]
    pub order_type: String,
    #[serde(rename="client_oid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

