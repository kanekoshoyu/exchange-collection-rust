#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SpotGridPushResponse represents a SpotGridPushResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SpotGridPushResponse {
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<SpotArgs>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SpotData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

