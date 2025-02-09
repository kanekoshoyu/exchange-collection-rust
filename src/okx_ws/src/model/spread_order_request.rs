#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SpreadOrderRequest represents a SpreadOrderRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SpreadOrderRequest {
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<SpreadOrderArg>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<SpreadOrderData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

