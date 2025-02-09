#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AccountSchema represents a AccountSchema model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountSchema {
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<Box<AccountEnum>>,
    #[serde(rename="E", skip_serializing_if = "Option::is_none")]
    pub reserved_e: Option<i32>,
    #[serde(rename="T", skip_serializing_if = "Option::is_none")]
    pub t: Option<bool>,
    #[serde(rename="W", skip_serializing_if = "Option::is_none")]
    pub w: Option<bool>,
    #[serde(rename="D", skip_serializing_if = "Option::is_none")]
    pub d: Option<bool>,
    #[serde(rename="B", skip_serializing_if = "Option::is_none")]
    pub b: Option<Vec<AccountArray>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

