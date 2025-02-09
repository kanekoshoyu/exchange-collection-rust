#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OpenInterestData represents a OpenInterestData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OpenInterestData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="oi", skip_serializing_if = "Option::is_none")]
    pub oi: Option<String>,
    #[serde(rename="oiCcy", skip_serializing_if = "Option::is_none")]
    pub oi_ccy: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

