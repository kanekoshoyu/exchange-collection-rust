#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FlexFuturesSchema represents a FlexFuturesSchema model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FlexFuturesSchema {
    #[serde(rename="currencies", skip_serializing_if = "Option::is_none")]
    pub currencies: Option<std::collections::HashMap<String, AnonymousSchema56>>,
    #[serde(rename="balance_value", skip_serializing_if = "Option::is_none")]
    pub balance_value: Option<f64>,
    #[serde(rename="portfolio_value", skip_serializing_if = "Option::is_none")]
    pub portfolio_value: Option<f64>,
    #[serde(rename="collateral_value", skip_serializing_if = "Option::is_none")]
    pub collateral_value: Option<f64>,
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
    #[serde(rename="margin_equity", skip_serializing_if = "Option::is_none")]
    pub margin_equity: Option<f64>,
    #[serde(rename="available_margin", skip_serializing_if = "Option::is_none")]
    pub available_margin: Option<f64>,
    #[serde(rename="isolated", skip_serializing_if = "Option::is_none")]
    pub isolated: Option<std::collections::HashMap<String, AnonymousSchema75>>,
    #[serde(rename="cross", skip_serializing_if = "Option::is_none")]
    pub cross: Option<Box<CrossSchema>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

