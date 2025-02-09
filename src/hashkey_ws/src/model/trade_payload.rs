#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradePayload represents a TradePayload model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradePayload {
    #[serde(rename="symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename="symbolName", skip_serializing_if = "Option::is_none")]
    pub symbol_name: Option<String>,
    #[serde(rename="topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(rename="params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Box<TradeParams>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TradeData>>,
    #[serde(rename="f", skip_serializing_if = "Option::is_none")]
    pub f: Option<bool>,
    #[serde(rename="sendTime", skip_serializing_if = "Option::is_none")]
    pub send_time: Option<i32>,
    #[serde(rename="shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(rename="id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

