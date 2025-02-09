#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Trades represents a Trades model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Trades {
    #[serde(rename="trade_id", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    #[serde(rename="product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename="price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename="size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

