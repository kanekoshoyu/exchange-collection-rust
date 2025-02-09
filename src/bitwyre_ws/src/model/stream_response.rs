#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// StreamResponse represents a StreamResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct StreamResponse {
    #[serde(rename="keys")]
    pub keys: Vec<String>,
    #[serde(rename="types")]
    pub types: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename="table")]
    pub table: String,
    #[serde(rename="action")]
    pub action: String,
    #[serde(rename="data")]
    pub data: Box<DepthL2Data>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

