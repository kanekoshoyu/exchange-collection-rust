#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// KlineStreamWithUtc represents a KlineStreamWithUtc model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct KlineStreamWithUtc {
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename="E", skip_serializing_if = "Option::is_none")]
    pub reserved_e: Option<i64>,
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="k", skip_serializing_if = "Option::is_none")]
    pub k: Option<Box<KlineInterval>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

