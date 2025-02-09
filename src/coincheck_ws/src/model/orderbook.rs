#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Orderbook represents a Orderbook model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Orderbook {
    #[serde(rename="bids", skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<Vec<String>>>,
    #[serde(rename="asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<Vec<String>>>,
    #[serde(rename="last_update_at", skip_serializing_if = "Option::is_none")]
    pub last_update_at: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

