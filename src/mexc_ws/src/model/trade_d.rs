#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeD represents a TradeD model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeD {
    #[serde(rename="deals", skip_serializing_if = "Option::is_none")]
    pub deals: Option<Vec<DealInfo>>,
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

