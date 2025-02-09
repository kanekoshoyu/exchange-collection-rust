/*
 * OKX API
 *
 * Welcome to OKX Developer document!   excluded below endpoints as they are if you need them please add and commit to https://github.com/kanekoshoyu/exchange-collection): - Trading Account (this might become needed, will add when we need it) - Block Trading - Financial Producer - Affiliate - Status - Announcement 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveSignalResponseExitSettingParam {
    /// Type of take-profit and stop-loss trigger: - `pnl`: Based on estimated profit and loss percentage from the entry point - `price`: Based on price increase or decrease from the entry price 
    #[serde(rename = "tpSlType", skip_serializing_if = "Option::is_none")]
    pub tp_sl_type: Option<TpSlType>,
    /// Take-profit percentage.
    #[serde(rename = "tpPct", skip_serializing_if = "Option::is_none")]
    pub tp_pct: Option<String>,
    /// Stop-loss percentage.
    #[serde(rename = "slPct", skip_serializing_if = "Option::is_none")]
    pub sl_pct: Option<String>,
}

impl ActiveSignalResponseExitSettingParam {
    pub fn new() -> ActiveSignalResponseExitSettingParam {
        ActiveSignalResponseExitSettingParam {
            tp_sl_type: None,
            tp_pct: None,
            sl_pct: None,
        }
    }
}
/// Type of take-profit and stop-loss trigger: - `pnl`: Based on estimated profit and loss percentage from the entry point - `price`: Based on price increase or decrease from the entry price 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TpSlType {
    #[serde(rename = "pnl")]
    Pnl,
    #[serde(rename = "price")]
    Price,
}

impl Default for TpSlType {
    fn default() -> TpSlType {
        Self::Pnl
    }
}

