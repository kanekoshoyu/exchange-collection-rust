#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SpreadCancelOrderRequest represents a SpreadCancelOrderRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SpreadCancelOrderRequest {
    #[serde(rename="id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename="op", skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[serde(rename="args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<SpreadCancelArgs>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

