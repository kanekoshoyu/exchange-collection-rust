#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// IntradayMarginWindowMeasure represents a IntradayMarginWindowMeasure model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct IntradayMarginWindowMeasure {
    #[serde(rename="margin_window_type", skip_serializing_if = "Option::is_none")]
    pub margin_window_type: Option<Box<MarginEnum>>,
    #[serde(rename="margin_level", skip_serializing_if = "Option::is_none")]
    pub margin_level: Option<Box<MarginLevelEnum>>,
    #[serde(rename="initial_margin", skip_serializing_if = "Option::is_none")]
    pub initial_margin: Option<String>,
    #[serde(rename="maintenance_margin", skip_serializing_if = "Option::is_none")]
    pub maintenance_margin: Option<String>,
    #[serde(rename="liquidation_buffer_percentage", skip_serializing_if = "Option::is_none")]
    pub liquidation_buffer_percentage: Option<String>,
    #[serde(rename="total_hold", skip_serializing_if = "Option::is_none")]
    pub total_hold: Option<String>,
    #[serde(rename="futures_buying_power", skip_serializing_if = "Option::is_none")]
    pub futures_buying_power: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

