#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PositionRequestArgs represents a PositionRequestArgs model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PositionRequestArgs {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<Box<InstTypeEnum>>,
    #[serde(rename="instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="extraParams", skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<Box<PositionExtraParams>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

