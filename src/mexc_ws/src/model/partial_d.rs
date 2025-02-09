#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PartialD represents a PartialD model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PartialD {
    #[serde(rename="asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<OrderLevel>>,
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

