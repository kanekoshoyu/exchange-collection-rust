#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PushResponse represents a PushResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PushResponse {
    #[serde(rename="action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<GeneralRequest>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Union>>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

