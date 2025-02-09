#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PositionsSnapshotResponse represents a PositionsSnapshotResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PositionsSnapshotResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="account")]
    pub account: String,
    #[serde(rename="positions")]
    pub positions: Vec<OpenPosition>,
    #[serde(rename="seq")]
    pub seq: i64,
    #[serde(rename="timestamp")]
    pub timestamp: i64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

