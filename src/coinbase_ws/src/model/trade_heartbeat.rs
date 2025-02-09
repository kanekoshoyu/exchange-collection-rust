#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeHeartbeat represents a TradeHeartbeat model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeHeartbeat {
    #[serde(rename="channel")]
    pub channel: String,
    #[serde(rename="client_id")]
    pub client_id: String,
    #[serde(rename="timestamp")]
    pub timestamp: String,
    #[serde(rename="sequence_num")]
    pub sequence_num: i32,
    #[serde(rename="events")]
    pub events: Vec<HeartbeatEnum>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

