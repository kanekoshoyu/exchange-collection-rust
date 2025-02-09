#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// GridPositionPushResponse represents a GridPositionPushResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct GridPositionPushResponse {
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<AlgoArg>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<GridItems>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

