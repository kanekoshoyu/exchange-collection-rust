#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MiniTickersData represents a MiniTickersData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MiniTickersData {
    #[serde(rename="c")]
    pub c: String,
    #[serde(rename="d")]
    pub d: Vec<MiniTickerInfo>,
    #[serde(rename="t")]
    pub t: i64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

