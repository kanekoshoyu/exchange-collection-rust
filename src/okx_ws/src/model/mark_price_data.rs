#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MarkPriceData represents a MarkPriceData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MarkPriceData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="markPx", skip_serializing_if = "Option::is_none")]
    pub mark_px: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

