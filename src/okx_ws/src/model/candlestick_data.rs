#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CandlestickData represents a CandlestickData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CandlestickData {
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="o", skip_serializing_if = "Option::is_none")]
    pub o: Option<String>,
    #[serde(rename="h", skip_serializing_if = "Option::is_none")]
    pub h: Option<String>,
    #[serde(rename="l", skip_serializing_if = "Option::is_none")]
    pub l: Option<String>,
    #[serde(rename="c", skip_serializing_if = "Option::is_none")]
    pub c: Option<String>,
    #[serde(rename="vol", skip_serializing_if = "Option::is_none")]
    pub vol: Option<String>,
    #[serde(rename="confirm", skip_serializing_if = "Option::is_none")]
    pub confirm: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

