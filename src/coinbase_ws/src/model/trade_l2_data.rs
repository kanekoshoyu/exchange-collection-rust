#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeL2Data represents a TradeL2Data model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeL2Data {
    #[serde(rename="channel")]
    pub channel: String,
    #[serde(rename="client_id")]
    pub client_id: String,
    #[serde(rename="timestamp")]
    pub timestamp: String,
    #[serde(rename="sequence_num")]
    pub sequence_num: i32,
    #[serde(rename="events")]
    pub events: Vec<L2Events>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

