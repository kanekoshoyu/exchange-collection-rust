#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Fill represents a Fill model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Fill {
    #[serde(rename="instrument")]
    pub instrument: String,
    #[serde(rename="time")]
    pub time: i64,
    #[serde(rename="price")]
    pub price: f64,
    #[serde(rename="seq")]
    pub seq: i64,
    #[serde(rename="buy")]
    pub buy: bool,
    #[serde(rename="qty")]
    pub qty: f64,
    #[serde(rename="remaining_order_qty")]
    pub remaining_order_qty: f64,
    #[serde(rename="order_id")]
    pub order_id: String,
    #[serde(rename="cli_ord_id", skip_serializing_if = "Option::is_none")]
    pub cli_ord_id: Option<String>,
    #[serde(rename="fill_id")]
    pub fill_id: String,
    #[serde(rename="fill_type")]
    pub fill_type: Box<FillEnum>,
    #[serde(rename="fee_paid", skip_serializing_if = "Option::is_none")]
    pub fee_paid: Option<f64>,
    #[serde(rename="fee_currency", skip_serializing_if = "Option::is_none")]
    pub fee_currency: Option<String>,
    #[serde(rename="taker_order_type", skip_serializing_if = "Option::is_none")]
    pub taker_order_type: Option<String>,
    #[serde(rename="order_type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

