#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PushDataResponse represents a PushDataResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PushDataResponse {
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Arg>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

