#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AnonymousSchema75 represents a AnonymousSchema75 model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AnonymousSchema75 {
    #[serde(rename="initial_margin", skip_serializing_if = "Option::is_none")]
    pub initial_margin: Option<f64>,
    #[serde(rename="initial_margin_without_orders", skip_serializing_if = "Option::is_none")]
    pub initial_margin_without_orders: Option<f64>,
    #[serde(rename="maintenance_margin", skip_serializing_if = "Option::is_none")]
    pub maintenance_margin: Option<f64>,
    #[serde(rename="pnl", skip_serializing_if = "Option::is_none")]
    pub pnl: Option<f64>,
    #[serde(rename="unrealized_funding", skip_serializing_if = "Option::is_none")]
    pub unrealized_funding: Option<f64>,
    #[serde(rename="total_unrealized", skip_serializing_if = "Option::is_none")]
    pub total_unrealized: Option<f64>,
    #[serde(rename="total_unrealized_as_margin", skip_serializing_if = "Option::is_none")]
    pub total_unrealized_as_margin: Option<f64>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

