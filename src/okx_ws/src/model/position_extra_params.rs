#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PositionExtraParams represents a PositionExtraParams model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PositionExtraParams {
    #[serde(rename="updateInterval", skip_serializing_if = "Option::is_none")]
    pub update_interval: Option<i32>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

