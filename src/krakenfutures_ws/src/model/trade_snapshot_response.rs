#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeSnapshotResponse represents a TradeSnapshotResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeSnapshotResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="product_id")]
    pub product_id: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

