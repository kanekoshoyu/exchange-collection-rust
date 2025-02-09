#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// BookData represents a BookData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BookData {
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<DepthItem>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

