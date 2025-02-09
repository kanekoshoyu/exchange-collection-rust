#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CancelData represents a CancelData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct CancelData {
    #[serde(rename="result", skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

