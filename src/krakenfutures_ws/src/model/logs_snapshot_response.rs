#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// LogsSnapshotResponse represents a LogsSnapshotResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LogsSnapshotResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="logs")]
    pub logs: Vec<AccountLogEntry>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

