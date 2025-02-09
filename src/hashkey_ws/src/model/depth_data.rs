#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// DepthData represents a DepthData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DepthData {
    #[serde(rename="e", skip_serializing_if = "Option::is_none")]
    pub e: Option<i32>,
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="t", skip_serializing_if = "Option::is_none")]
    pub t: Option<i32>,
    #[serde(rename="v", skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    #[serde(rename="b", skip_serializing_if = "Option::is_none")]
    pub b: Option<Vec<Vec<String>>>,
    #[serde(rename="a", skip_serializing_if = "Option::is_none")]
    pub a: Option<Vec<Vec<String>>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

