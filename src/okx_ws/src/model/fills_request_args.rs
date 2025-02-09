#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FillsRequestArgs represents a FillsRequestArgs model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FillsRequestArgs {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

