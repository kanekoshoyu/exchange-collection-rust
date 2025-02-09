#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// DepthItem represents a DepthItem model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DepthItem {
    #[serde(rename="asks")]
    pub asks: Vec<Vec<String>>,
    #[serde(rename="bids")]
    pub bids: Vec<Vec<String>>,
    #[serde(rename="ts")]
    pub ts: String,
    #[serde(rename="checksum")]
    pub checksum: i32,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

