#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PartialDepthData represents a PartialDepthData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PartialDepthData {
    #[serde(rename="c")]
    pub c: String,
    #[serde(rename="d")]
    pub d: Box<PartialD>,
    #[serde(rename="r")]
    pub r: String,
    #[serde(rename="s")]
    pub s: String,
    #[serde(rename="t")]
    pub t: i64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

