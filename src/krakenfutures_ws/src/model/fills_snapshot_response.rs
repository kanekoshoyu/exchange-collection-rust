#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FillsSnapshotResponse represents a FillsSnapshotResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FillsSnapshotResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="account")]
    pub account: String,
    #[serde(rename="fills")]
    pub fills: Vec<Fill>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

