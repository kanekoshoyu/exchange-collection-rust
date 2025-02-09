#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ErrorResponse represents a ErrorResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ErrorResponse {
    #[serde(rename="event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(rename="code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename="msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

