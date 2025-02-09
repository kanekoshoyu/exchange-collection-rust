#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ChatRequest represents a ChatRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ChatRequest {
    #[serde(rename="username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

