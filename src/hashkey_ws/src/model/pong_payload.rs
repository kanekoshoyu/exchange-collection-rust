#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PongPayload represents a PongPayload model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PongPayload {
    #[serde(rename="pong", skip_serializing_if = "Option::is_none")]
    pub pong: Option<i32>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

