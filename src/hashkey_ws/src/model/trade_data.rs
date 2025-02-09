#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeData represents a TradeData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TradeData {
    #[serde(rename="v", skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    #[serde(rename="t", skip_serializing_if = "Option::is_none")]
    pub t: Option<i32>,
    #[serde(rename="p", skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(rename="q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename="m", skip_serializing_if = "Option::is_none")]
    pub m: Option<bool>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

