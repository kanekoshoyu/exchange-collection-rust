#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FillsPushDataArg represents a FillsPushDataArg model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FillsPushDataArg {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

