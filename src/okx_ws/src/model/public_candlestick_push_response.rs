#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PublicCandlestickPushResponse represents a PublicCandlestickPushResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PublicCandlestickPushResponse {
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<SprdArg>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<CandlestickData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

