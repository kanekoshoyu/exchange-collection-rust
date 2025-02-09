#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderLevel represents a OrderLevel model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderLevel {
    #[serde(rename="p")]
    pub p: String,
    #[serde(rename="v")]
    pub v: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

