#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OpenOrdersDeltaResponse represents a OpenOrdersDeltaResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OpenOrdersDeltaResponse {
    #[serde(rename="feed")]
    pub feed: Box<FeedEnum>,
    #[serde(rename="order")]
    pub order: Box<Order>,
    #[serde(rename="is_cancel")]
    pub is_cancel: bool,
    #[serde(rename="reason")]
    pub reason: Box<ReasonEnum>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

