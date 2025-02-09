#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Bids represents a Bids model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Bids {
    #[serde(rename="qty")]
    pub qty: f64,
    #[serde(rename="price")]
    pub price: f64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

