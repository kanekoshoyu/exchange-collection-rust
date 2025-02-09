#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AccountArray represents a AccountArray model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountArray {
    #[serde(rename="a", skip_serializing_if = "Option::is_none")]
    pub a: Option<String>,
    #[serde(rename="f", skip_serializing_if = "Option::is_none")]
    pub f: Option<String>,
    #[serde(rename="l", skip_serializing_if = "Option::is_none")]
    pub l: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

