#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AccountLogEntry represents a AccountLogEntry model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AccountLogEntry {
    #[serde(rename="id")]
    pub id: i64,
    #[serde(rename="date")]
    pub date: String,
    #[serde(rename="asset")]
    pub asset: String,
    #[serde(rename="info")]
    pub info: String,
    #[serde(rename="booking_uid")]
    pub booking_uid: String,
    #[serde(rename="margin_account")]
    pub margin_account: String,
    #[serde(rename="old_balance")]
    pub old_balance: f64,
    #[serde(rename="new_balance")]
    pub new_balance: f64,
    #[serde(rename="old_average_entry_price")]
    pub old_average_entry_price: f64,
    #[serde(rename="new_average_entry_price")]
    pub new_average_entry_price: f64,
    #[serde(rename="trade_price")]
    pub trade_price: f64,
    #[serde(rename="mark_price")]
    pub mark_price: f64,
    #[serde(rename="realized_pnl")]
    pub realized_pnl: f64,
    #[serde(rename="fee")]
    pub fee: f64,
    #[serde(rename="execution")]
    pub execution: String,
    #[serde(rename="collateral")]
    pub collateral: String,
    #[serde(rename="funding_rate")]
    pub funding_rate: f64,
    #[serde(rename="realized_funding")]
    pub realized_funding: f64,
    #[serde(rename="conversion_spread_percentage")]
    pub conversion_spread_percentage: f64,
    #[serde(rename="liquidation_fee")]
    pub liquidation_fee: f64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

