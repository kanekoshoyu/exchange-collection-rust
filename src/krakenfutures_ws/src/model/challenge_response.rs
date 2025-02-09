#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ChallengeResponse represents a ChallengeResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ChallengeResponse {
    #[serde(rename="event")]
    pub event: Box<EventEnum>,
    #[serde(rename="message")]
    pub message: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

