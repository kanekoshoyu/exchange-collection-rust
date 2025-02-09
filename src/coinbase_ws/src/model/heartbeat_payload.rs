#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// HeartbeatPayload represents a HeartbeatPayload model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HeartbeatPayload {
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub reserved_type: Option<String>,
    #[serde(rename="sequence", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
    #[serde(rename="last_trade_id", skip_serializing_if = "Option::is_none")]
    pub last_trade_id: Option<i32>,
    #[serde(rename="product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename="time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

