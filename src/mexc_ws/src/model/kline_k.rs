#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// KlineK represents a KlineK model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct KlineK {
    #[serde(rename="k", skip_serializing_if = "Option::is_none")]
    pub k: Option<Box<KlineInfo>>,
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

