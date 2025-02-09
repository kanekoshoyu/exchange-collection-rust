#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// BookDeltaResponse represents a BookDeltaResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BookDeltaResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="seq")]
    pub seq: i32,
    #[serde(rename="timestamp")]
    pub timestamp: i32,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="price")]
    pub price: f64,
    #[serde(rename="qty")]
    pub qty: f64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

