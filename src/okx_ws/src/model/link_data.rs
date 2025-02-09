#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// LinkData represents a LinkData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LinkData {
    #[serde(rename="ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

