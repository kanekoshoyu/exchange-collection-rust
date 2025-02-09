#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SentMessage represents a SentMessage model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SentMessage {
    #[serde(rename="timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
    #[serde(rename="message_id", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

