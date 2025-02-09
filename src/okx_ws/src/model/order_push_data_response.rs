#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderPushDataResponse represents a OrderPushDataResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderPushDataResponse {
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<OrderPushDataArg>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OrderPushData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

