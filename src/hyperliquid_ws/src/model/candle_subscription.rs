#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CandleSubscription represents a CandleSubscription model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CandleSubscription {
    #[serde(rename="type")]
    pub reserved_type: Box<CandleEnum>,
    #[serde(rename="coin")]
    pub coin: String,
    #[serde(rename="interval")]
    pub interval: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

