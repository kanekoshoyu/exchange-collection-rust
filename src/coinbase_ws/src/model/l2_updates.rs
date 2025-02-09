#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// L2Updates represents a L2Updates model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct L2Updates {
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="event_time", skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    #[serde(rename="price_level", skip_serializing_if = "Option::is_none")]
    pub price_level: Option<String>,
    #[serde(rename="new_quantity", skip_serializing_if = "Option::is_none")]
    pub new_quantity: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

