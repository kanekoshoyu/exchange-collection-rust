#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// BalancePushDataResponse represents a BalancePushDataResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BalancePushDataResponse {
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<BalancePushDataArg>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<BalancePushData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

