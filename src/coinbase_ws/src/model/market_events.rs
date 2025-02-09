#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MarketEvents represents a MarketEvents model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MarketEvents {
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub reserved_type: Option<String>,
    #[serde(rename="trades", skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<Trades>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

