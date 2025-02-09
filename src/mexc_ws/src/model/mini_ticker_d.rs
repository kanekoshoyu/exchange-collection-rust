#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MiniTickerD represents a MiniTickerD model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MiniTickerD {
    #[serde(rename="s", skip_serializing_if = "Option::is_none")]
    pub s: Option<String>,
    #[serde(rename="p", skip_serializing_if = "Option::is_none")]
    pub p: Option<String>,
    #[serde(rename="r", skip_serializing_if = "Option::is_none")]
    pub r: Option<String>,
    #[serde(rename="tr", skip_serializing_if = "Option::is_none")]
    pub tr: Option<String>,
    #[serde(rename="h", skip_serializing_if = "Option::is_none")]
    pub h: Option<String>,
    #[serde(rename="l", skip_serializing_if = "Option::is_none")]
    pub l: Option<String>,
    #[serde(rename="v", skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
    #[serde(rename="q", skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(rename="lastRT", skip_serializing_if = "Option::is_none")]
    pub last_rt: Option<String>,
    #[serde(rename="MT", skip_serializing_if = "Option::is_none")]
    pub mt: Option<String>,
    #[serde(rename="NV", skip_serializing_if = "Option::is_none")]
    pub nv: Option<String>,
    #[serde(rename="t", skip_serializing_if = "Option::is_none")]
    pub t: Option<i64>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

