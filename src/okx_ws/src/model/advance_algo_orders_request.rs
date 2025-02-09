#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AdvanceAlgoOrdersRequest represents a AdvanceAlgoOrdersRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AdvanceAlgoOrdersRequest {
    #[serde(rename="op", skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[serde(rename="args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<DepositArg>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

