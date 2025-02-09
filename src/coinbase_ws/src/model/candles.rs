#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Candles represents a Candles model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Candles {
    #[serde(rename="start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename="high", skip_serializing_if = "Option::is_none")]
    pub high: Option<String>,
    #[serde(rename="low", skip_serializing_if = "Option::is_none")]
    pub low: Option<String>,
    #[serde(rename="open", skip_serializing_if = "Option::is_none")]
    pub open: Option<String>,
    #[serde(rename="close", skip_serializing_if = "Option::is_none")]
    pub close: Option<String>,
    #[serde(rename="volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<String>,
    #[serde(rename="product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

