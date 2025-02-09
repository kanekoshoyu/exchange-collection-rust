#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TickerEvents represents a TickerEvents model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TickerEvents {
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub reserved_type: Option<String>,
    #[serde(rename="tickers", skip_serializing_if = "Option::is_none")]
    pub tickers: Option<Vec<TickersArray>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

