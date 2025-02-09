#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CancelOrderRequest represents a CancelOrderRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CancelOrderRequest {
    #[serde(rename="id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename="op", skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[serde(rename="args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<CancelOrderArgs>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

