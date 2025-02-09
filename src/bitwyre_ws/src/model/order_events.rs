#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderEvents represents a OrderEvents model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderEvents {
    #[serde(rename="instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

