#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PublishedMessage represents a PublishedMessage model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PublishedMessage {
    #[serde(rename="op", skip_serializing_if = "Option::is_none")]
    pub op: Option<Box<OpEnum>>,
    #[serde(rename="args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<Args>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

