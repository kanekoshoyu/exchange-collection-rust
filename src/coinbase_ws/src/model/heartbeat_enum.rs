#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// HeartbeatEnum represents a HeartbeatEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HeartbeatEnum {
    #[serde(rename="current_time", skip_serializing_if = "Option::is_none")]
    pub current_time: Option<String>,
    #[serde(rename="heartbeat_counter", skip_serializing_if = "Option::is_none")]
    pub heartbeat_counter: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

