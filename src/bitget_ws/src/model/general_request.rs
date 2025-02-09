#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// GeneralRequest represents a GeneralRequest model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct GeneralRequest {
    #[serde(rename="InstType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="InstId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<Box<ChannelEnum>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

