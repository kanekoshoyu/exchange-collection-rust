#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Details represents a Details model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Details {
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="eq", skip_serializing_if = "Option::is_none")]
    pub eq: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

