#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// InstrumentPushResponse represents a InstrumentPushResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct InstrumentPushResponse {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<InstrumentData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

