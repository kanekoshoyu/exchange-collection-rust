#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OptionPushResponse represents a OptionPushResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OptionPushResponse {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OptionData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

