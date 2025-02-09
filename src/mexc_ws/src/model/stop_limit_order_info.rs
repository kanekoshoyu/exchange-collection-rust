#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// StopLimitOrderInfo represents a StopLimitOrderInfo model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct StopLimitOrderInfo {
    #[serde(rename="N")]
    pub n: String,
    #[serde(rename="o")]
    pub o: i32,
    #[serde(rename="p")]
    pub p: String,
    #[serde(rename="s")]
    pub s: String,
    #[serde(rename="T")]
    pub t: i32,
    #[serde(rename="i")]
    pub i: String,
    #[serde(rename="v")]
    pub v: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

