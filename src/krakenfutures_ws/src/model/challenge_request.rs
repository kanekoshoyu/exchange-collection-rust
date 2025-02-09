#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ChallengeRequest represents a ChallengeRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ChallengeRequest {
    #[serde(rename="event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(rename="api_key", skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

