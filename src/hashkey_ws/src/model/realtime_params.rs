#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// RealtimeParams represents a RealtimeParams model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RealtimeParams {
    #[serde(rename="realtimeInterval", skip_serializing_if = "Option::is_none")]
    pub realtime_interval: Option<String>,
    #[serde(rename="binary", skip_serializing_if = "Option::is_none")]
    pub binary: Option<bool>,
    #[serde(rename="dumpScale", skip_serializing_if = "Option::is_none")]
    pub dump_scale: Option<i32>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

