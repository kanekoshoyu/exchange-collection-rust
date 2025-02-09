#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// LoginRequest represents a LoginRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LoginRequest {
    #[serde(rename="apiKey")]
    pub api_key: String,
    #[serde(rename="passphrase")]
    pub passphrase: String,
    #[serde(rename="timestamp")]
    pub timestamp: String,
    #[serde(rename="sign")]
    pub sign: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

