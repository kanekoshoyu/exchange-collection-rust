#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OptionTradesResponse represents a OptionTradesResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OptionTradesResponse {
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<OptionObj>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

