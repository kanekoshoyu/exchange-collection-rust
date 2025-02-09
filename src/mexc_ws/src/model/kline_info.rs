#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// KlineInfo represents a KlineInfo model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct KlineInfo {
    #[serde(rename="t")]
    pub t: i64,
    #[serde(rename="a")]
    pub a: String,
    #[serde(rename="c")]
    pub c: String,
    #[serde(rename="h")]
    pub h: String,
    #[serde(rename="i")]
    pub i: String,
    #[serde(rename="l")]
    pub l: String,
    #[serde(rename="o")]
    pub o: String,
    #[serde(rename="v")]
    pub v: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

