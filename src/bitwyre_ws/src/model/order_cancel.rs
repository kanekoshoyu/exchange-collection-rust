#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderCancel represents a OrderCancel model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OrderCancel {
    #[serde(rename="order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename="qtys", skip_serializing_if = "Option::is_none")]
    pub qtys: Option<i32>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

