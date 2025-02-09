#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Data represents a Data model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Data {
    #[serde(rename="uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    #[serde(rename="totalEq", skip_serializing_if = "Option::is_none")]
    pub total_eq: Option<String>,
    #[serde(rename="details", skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<Details>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

