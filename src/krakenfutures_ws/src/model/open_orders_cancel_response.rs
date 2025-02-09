#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OpenOrdersCancelResponse represents a OpenOrdersCancelResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OpenOrdersCancelResponse {
    #[serde(rename="feed")]
    pub feed: Box<FeedEnum>,
    #[serde(rename="order_id")]
    pub order_id: String,
    #[serde(rename="is_cancel")]
    pub is_cancel: bool,
    #[serde(rename="reason")]
    pub reason: Box<ReasonEnum>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

