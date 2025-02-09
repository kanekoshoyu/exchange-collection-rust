#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TickersD represents a TickersD model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TickersD {
    #[serde(rename="a", skip_serializing_if = "Option::is_none")]
    pub a: Option<String>,
    #[serde(rename="b", skip_serializing_if = "Option::is_none")]
    pub b: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

