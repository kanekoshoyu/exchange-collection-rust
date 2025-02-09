#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MiniTickerData represents a MiniTickerData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MiniTickerData {
    #[serde(rename="c")]
    pub c: String,
    #[serde(rename="d")]
    pub d: Box<MiniTickerD>,
    #[serde(rename="t")]
    pub t: i64,
    #[serde(rename="s")]
    pub s: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

