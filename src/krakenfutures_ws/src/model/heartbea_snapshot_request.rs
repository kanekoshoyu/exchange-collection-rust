#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// HeartbeaSnapshotRequest represents a HeartbeaSnapshotRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HeartbeaSnapshotRequest {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="time")]
    pub time: i32,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

