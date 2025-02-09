#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// KlineData represents a KlineData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct KlineData {
    #[serde(rename="c")]
    pub c: String,
    #[serde(rename="d")]
    pub d: Box<KlineK>,
    #[serde(rename="s")]
    pub s: String,
    #[serde(rename="t")]
    pub t: i64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

