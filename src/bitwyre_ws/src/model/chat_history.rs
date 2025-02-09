#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ChatHistory represents a ChatHistory model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ChatHistory {
    #[serde(rename="username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename="chats", skip_serializing_if = "Option::is_none")]
    pub chats: Option<Vec<serde_json::Value>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

