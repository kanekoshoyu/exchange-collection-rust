#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ContractGridPushResponse represents a ContractGridPushResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ContractGridPushResponse {
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<Arg>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ContractData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

