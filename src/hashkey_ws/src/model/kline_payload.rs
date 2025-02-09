#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// KlinePayload represents a KlinePayload model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct KlinePayload {
    #[serde(rename="symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename="symbolName", skip_serializing_if = "Option::is_none")]
    pub symbol_name: Option<String>,
    #[serde(rename="topic", skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(rename="params", skip_serializing_if = "Option::is_none")]
    pub params: Option<Box<KlineParams>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<KlineData>>,
    #[serde(rename="f", skip_serializing_if = "Option::is_none")]
    pub f: Option<bool>,
    #[serde(rename="sendTime", skip_serializing_if = "Option::is_none")]
    pub send_time: Option<i32>,
    #[serde(rename="id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

