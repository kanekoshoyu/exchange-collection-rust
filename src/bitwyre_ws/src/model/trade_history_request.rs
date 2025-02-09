#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeHistoryRequest represents a TradeHistoryRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeHistoryRequest {
    #[serde(rename="instrument")]
    pub instrument: String,
    #[serde(rename="count")]
    pub count: i32,
    #[serde(rename="from_time", skip_serializing_if = "Option::is_none")]
    pub from_time: Option<i64>,
    #[serde(rename="to_time", skip_serializing_if = "Option::is_none")]
    pub to_time: Option<i64>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

