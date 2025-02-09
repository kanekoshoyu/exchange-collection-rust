#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// LiquidatedPosition represents a LiquidatedPosition model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LiquidatedPosition {
    #[serde(rename="coin")]
    pub coin: String,
    #[serde(rename="szi")]
    pub szi: f64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

