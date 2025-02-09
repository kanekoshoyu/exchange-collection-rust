#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Arg represents a Arg model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Arg {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<PushDataDetails>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

