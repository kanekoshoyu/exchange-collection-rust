#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// RecurringOrderResponse represents a RecurringOrderResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RecurringOrderResponse {
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<RecurringOrderArg>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<RecurringOrderData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

