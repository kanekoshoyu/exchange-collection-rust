#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// DealInfo represents a DealInfo model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DealInfo {
    #[serde(rename="S")]
    pub s: i32,
    #[serde(rename="p")]
    pub p: String,
    #[serde(rename="t")]
    pub t: i64,
    #[serde(rename="v")]
    pub v: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

