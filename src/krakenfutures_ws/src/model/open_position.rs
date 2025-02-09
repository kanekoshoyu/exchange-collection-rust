#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OpenPosition represents a OpenPosition model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OpenPosition {
    #[serde(rename="instrument")]
    pub instrument: String,
    #[serde(rename="balance")]
    pub balance: f64,
    #[serde(rename="pnl")]
    pub pnl: f64,
    #[serde(rename="entry_price")]
    pub entry_price: f64,
    #[serde(rename="mark_price")]
    pub mark_price: f64,
    #[serde(rename="index_price")]
    pub index_price: f64,
    #[serde(rename="liquidation_threshold")]
    pub liquidation_threshold: f64,
    #[serde(rename="effective_leverage")]
    pub effective_leverage: f64,
    #[serde(rename="return_on_equity")]
    pub return_on_equity: f64,
    #[serde(rename="initial_margin")]
    pub initial_margin: f64,
    #[serde(rename="initial_margin_with_orders", skip_serializing_if = "Option::is_none")]
    pub initial_margin_with_orders: Option<f64>,
    #[serde(rename="maintenance_margin")]
    pub maintenance_margin: f64,
    #[serde(rename="pnl_currency", skip_serializing_if = "Option::is_none")]
    pub pnl_currency: Option<String>,
    #[serde(rename="unrealized_funding", skip_serializing_if = "Option::is_none")]
    pub unrealized_funding: Option<f64>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

