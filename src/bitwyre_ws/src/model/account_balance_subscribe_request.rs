#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AccountBalanceSubscribeRequest represents a AccountBalanceSubscribeRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountBalanceSubscribeRequest {
    #[serde(rename="command")]
    pub command: String,
    #[serde(rename="nonce")]
    pub nonce: i32,
    #[serde(rename="checksum")]
    pub checksum: String,
    #[serde(rename="api_key")]
    pub api_key: String,
    #[serde(rename="api_sign")]
    pub api_sign: String,
    #[serde(rename="payload", skip_serializing_if = "Option::is_none")]
    pub payload: Option<Box<PayloadUnion>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

