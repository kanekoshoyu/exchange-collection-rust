#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AnonymousSchema101 represents a AnonymousSchema101 model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AnonymousSchema101 {
    #[serde(rename="name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename="pair", skip_serializing_if = "Option::is_none")]
    pub pair: Option<String>,
    #[serde(rename="unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename="portfolio_value", skip_serializing_if = "Option::is_none")]
    pub portfolio_value: Option<f64>,
    #[serde(rename="balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    #[serde(rename="maintenance_margin", skip_serializing_if = "Option::is_none")]
    pub maintenance_margin: Option<f64>,
    #[serde(rename="initial_margin", skip_serializing_if = "Option::is_none")]
    pub initial_margin: Option<f64>,
    #[serde(rename="available", skip_serializing_if = "Option::is_none")]
    pub available: Option<f64>,
    #[serde(rename="unrealized_funding", skip_serializing_if = "Option::is_none")]
    pub unrealized_funding: Option<f64>,
    #[serde(rename="pnl", skip_serializing_if = "Option::is_none")]
    pub pnl: Option<f64>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

