#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// L2Events represents a L2Events model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct L2Events {
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub reserved_type: Option<String>,
    #[serde(rename="product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename="updates", skip_serializing_if = "Option::is_none")]
    pub updates: Option<Vec<L2Updates>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

