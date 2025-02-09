#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PlaceOrderRequest represents a PlaceOrderRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PlaceOrderRequest {
    #[serde(rename="id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename="op", skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[serde(rename="args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<PlaceOrderArgs>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

