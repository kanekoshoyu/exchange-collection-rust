#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeDeltaResponse represents a TradeDeltaResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeDeltaResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="uid")]
    pub uid: String,
    #[serde(rename="side")]
    pub side: Box<SideEnum>,
    #[serde(rename="type")]
    pub reserved_type: Box<TypeEnum>,
    #[serde(rename="seq")]
    pub seq: i32,
    #[serde(rename="time")]
    pub time: i32,
    #[serde(rename="qty")]
    pub qty: f64,
    #[serde(rename="price")]
    pub price: f64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

