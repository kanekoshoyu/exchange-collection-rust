#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AccountD represents a AccountD model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountD {
    #[serde(rename="a", skip_serializing_if = "Option::is_none")]
    pub a: Option<String>,
    #[serde(rename="c", skip_serializing_if = "Option::is_none")]
    pub c: Option<i64>,
    #[serde(rename="f", skip_serializing_if = "Option::is_none")]
    pub f: Option<String>,
    #[serde(rename="fd", skip_serializing_if = "Option::is_none")]
    pub fd: Option<String>,
    #[serde(rename="l", skip_serializing_if = "Option::is_none")]
    pub l: Option<String>,
    #[serde(rename="ld", skip_serializing_if = "Option::is_none")]
    pub ld: Option<String>,
    #[serde(rename="o", skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

