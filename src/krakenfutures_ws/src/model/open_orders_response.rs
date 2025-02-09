#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OpenOrdersResponse represents a OpenOrdersResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OpenOrdersResponse {
    #[serde(rename="event", skip_serializing_if = "Option::is_none")]
    pub event: Option<Box<EventEnum>>,
    #[serde(rename="feed", skip_serializing_if = "Option::is_none")]
    pub feed: Option<Box<FeedEnum>>,
    #[serde(rename="api_key", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename="original_challenge", skip_serializing_if = "Option::is_none")]
    pub original_challenge: Option<String>,
    #[serde(rename="signed_challenge", skip_serializing_if = "Option::is_none")]
    pub signed_challenge: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

