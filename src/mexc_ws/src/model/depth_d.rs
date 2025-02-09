#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// DepthD represents a DepthD model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DepthD {
    #[serde(rename="asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<OrderLevel>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

