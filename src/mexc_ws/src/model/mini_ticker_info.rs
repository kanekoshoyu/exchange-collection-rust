#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MiniTickerInfo represents a MiniTickerInfo model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MiniTickerInfo {
    #[serde(rename="s")]
    pub s: String,
    #[serde(rename="p")]
    pub p: String,
    #[serde(rename="r")]
    pub r: String,
    #[serde(rename="tr")]
    pub tr: String,
    #[serde(rename="h")]
    pub h: String,
    #[serde(rename="l")]
    pub l: String,
    #[serde(rename="v")]
    pub v: String,
    #[serde(rename="q")]
    pub q: String,
    #[serde(rename="lastRT")]
    pub last_rt: String,
    #[serde(rename="MT")]
    pub mt: String,
    #[serde(rename="NV")]
    pub nv: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

