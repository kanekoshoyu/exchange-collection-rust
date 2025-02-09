#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TrailingStopOptions represents a TrailingStopOptions model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TrailingStopOptions {
    #[serde(rename="max_deviation")]
    pub max_deviation: f64,
    #[serde(rename="unit")]
    pub unit: Box<UnitEnum>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

