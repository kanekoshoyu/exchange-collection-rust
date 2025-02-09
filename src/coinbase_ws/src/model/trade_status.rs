#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeStatus represents a TradeStatus model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeStatus {
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub reserved_type: Option<String>,
    #[serde(rename="products", skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<TradeProducts>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

