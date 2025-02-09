#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MassCancelRequest represents a MassCancelRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MassCancelRequest {
    #[serde(rename="id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename="op", skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[serde(rename="args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<MassCancelArgs>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

