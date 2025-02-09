#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FcmBalanceSummary represents a FcmBalanceSummary model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FcmBalanceSummary {
    #[serde(rename="futures_buying_power", skip_serializing_if = "Option::is_none")]
    pub futures_buying_power: Option<String>,
    #[serde(rename="total_usd_balance", skip_serializing_if = "Option::is_none")]
    pub total_usd_balance: Option<String>,
    #[serde(rename="cbi_usd_balance", skip_serializing_if = "Option::is_none")]
    pub cbi_usd_balance: Option<String>,
    #[serde(rename="cfm_usd_balance", skip_serializing_if = "Option::is_none")]
    pub cfm_usd_balance: Option<String>,
    #[serde(rename="total_open_orders_hold_amount", skip_serializing_if = "Option::is_none")]
    pub total_open_orders_hold_amount: Option<String>,
    #[serde(rename="unrealized_pnl", skip_serializing_if = "Option::is_none")]
    pub unrealized_pnl: Option<String>,
    #[serde(rename="daily_realized_pnl", skip_serializing_if = "Option::is_none")]
    pub daily_realized_pnl: Option<String>,
    #[serde(rename="initial_margin", skip_serializing_if = "Option::is_none")]
    pub initial_margin: Option<String>,
    #[serde(rename="available_margin", skip_serializing_if = "Option::is_none")]
    pub available_margin: Option<String>,
    #[serde(rename="liquidation_threshold", skip_serializing_if = "Option::is_none")]
    pub liquidation_threshold: Option<String>,
    #[serde(rename="liquidation_buffer_amount", skip_serializing_if = "Option::is_none")]
    pub liquidation_buffer_amount: Option<String>,
    #[serde(rename="liquidation_buffer_percentage", skip_serializing_if = "Option::is_none")]
    pub liquidation_buffer_percentage: Option<String>,
    #[serde(rename="intraday_margin_window_measure", skip_serializing_if = "Option::is_none")]
    pub intraday_margin_window_measure: Option<Box<IntradayMarginWindowMeasure>>,
    #[serde(rename="overnight_margin_window_measure", skip_serializing_if = "Option::is_none")]
    pub overnight_margin_window_measure: Option<Box<OvernightMarginWindowMeasure>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

