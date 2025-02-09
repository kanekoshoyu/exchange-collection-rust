#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradesSubscription represents a TradesSubscription model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradesSubscription {
    #[serde(rename="type")]
    pub reserved_type: Box<TradeEnum>,
    #[serde(rename="coin")]
    pub coin: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

