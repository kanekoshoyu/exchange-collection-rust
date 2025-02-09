#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// LogsDeltaResponse represents a LogsDeltaResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LogsDeltaResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="new_entry")]
    pub new_entry: Box<AccountLogEntry>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

