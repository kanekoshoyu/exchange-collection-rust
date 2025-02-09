#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// DoneOrders represents a DoneOrders model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DoneOrders {
    #[serde(rename="type", skip_serializing_if = "Option::is_none")]
    pub reserved_type: Option<String>,
    #[serde(rename="time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename="product_id", skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename="sequence", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i32>,
    #[serde(rename="price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(rename="order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename="reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<Box<ReasonEnum>>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="remaining_size", skip_serializing_if = "Option::is_none")]
    pub remaining_size: Option<String>,
    #[serde(rename="cancel_reason", skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<Box<CancelReason>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

