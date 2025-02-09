#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// UnsubscribeRequest represents a UnsubscribeRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UnsubscribeRequest {
    #[serde(rename="method")]
    pub method: String,
    #[serde(rename="params")]
    pub params: Vec<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

