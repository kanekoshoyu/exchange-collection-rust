#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MassCancelArgs represents a MassCancelArgs model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MassCancelArgs {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<Box<InstTypeEnum>>,
    #[serde(rename="instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(rename="lockInterval", skip_serializing_if = "Option::is_none")]
    pub lock_interval: Option<i32>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

