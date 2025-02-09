#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PingRequest represents a PingRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PingRequest {
    #[serde(rename="method")]
    pub method: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

