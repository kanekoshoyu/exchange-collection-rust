#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// KlineData represents a KlineData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct KlineData {
    #[serde(rename="t", skip_serializing_if = "Option::is_none")]
    pub t: Option<i32>,
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="sn", skip_serializing_if = "Option::is_none")]
    pub sn: Option<String>,
    #[serde(rename="c", skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(rename="h", skip_serializing_if = "Option::is_none")]
    pub h: Option<String>,
    #[serde(rename="l", skip_serializing_if = "Option::is_none")]
    pub l: Option<String>,
    #[serde(rename="o", skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    #[serde(rename="v", skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

