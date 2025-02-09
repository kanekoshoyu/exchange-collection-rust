#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Order represents a Order model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Order {
    #[serde(rename="instrument")]
    pub instrument: String,
    #[serde(rename="time")]
    pub time: i64,
    #[serde(rename="last_update_time")]
    pub last_update_time: i64,
    #[serde(rename="qty")]
    pub qty: f64,
    #[serde(rename="filled")]
    pub filled: f64,
    #[serde(rename="limit_price", skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f64>,
    #[serde(rename="stop_price", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,
    #[serde(rename="type")]
    pub reserved_type: Box<TypeEnum>,
    #[serde(rename="order_id")]
    pub order_id: String,
    #[serde(rename="direction")]
    pub direction: Box<DirectionEnum>,
    #[serde(rename="reduce_only")]
    pub reduce_only: bool,
    #[serde(rename="triggerSignal", skip_serializing_if = "Option::is_none")]
    pub trigger_signal: Option<Box<SignalEnum>>,
    #[serde(rename="trailing_stop_options", skip_serializing_if = "Option::is_none")]
    pub trailing_stop_options: Option<Box<TrailingStopOptions>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

