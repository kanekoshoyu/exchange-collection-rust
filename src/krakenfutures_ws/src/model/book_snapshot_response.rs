#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// BookSnapshotResponse represents a BookSnapshotResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BookSnapshotResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="seq")]
    pub seq: i32,
    #[serde(rename="timestamp")]
    pub timestamp: i32,
    #[serde(rename="tickSize", skip_serializing_if = "Option::is_none")]
    pub tick_size: Option<String>,
    #[serde(rename="bids")]
    pub bids: Vec<Bids>,
    #[serde(rename="asks")]
    pub asks: Vec<Bids>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

