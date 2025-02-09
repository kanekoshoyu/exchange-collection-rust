#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// DepthL2Order represents a DepthL2Order model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DepthL2Order {
    #[serde(rename="price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename="qty", skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

