#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// DepositRequest represents a DepositRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DepositRequest {
    #[serde(rename="op", skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[serde(rename="args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<DepositRequestArgs>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

