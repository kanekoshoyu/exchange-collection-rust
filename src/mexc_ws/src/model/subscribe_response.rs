#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SubscribeResponse represents a SubscribeResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscribeResponse {
    #[serde(rename="id")]
    pub id: i32,
    #[serde(rename="code")]
    pub code: i32,
    #[serde(rename="msg")]
    pub msg: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

