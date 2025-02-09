#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// HeartbeatRequest represents a HeartbeatRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HeartbeatRequest {
    #[serde(rename="event")]
    pub event: Box<EventEnum>,
    #[serde(rename="feed", skip_serializing_if = "Option::is_none")]
    pub feed: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

