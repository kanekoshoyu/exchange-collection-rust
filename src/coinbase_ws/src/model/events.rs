#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Events represents a Events model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Events {
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub reserved_type: Option<String>,
    #[serde(rename="candles", skip_serializing_if = "Option::is_none")]
    pub candles: Option<Vec<Candles>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

