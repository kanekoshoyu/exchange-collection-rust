/*
 * Bitget API
 *
 * Welcome to Bitget Developer document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeverageResponse {
    /// Trading pair
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Margin coin (e.g., USDT)
    #[serde(rename = "marginCoin", skip_serializing_if = "Option::is_none")]
    pub margin_coin: Option<String>,
    /// Leverage for long positions
    #[serde(rename = "longLeverage", skip_serializing_if = "Option::is_none")]
    pub long_leverage: Option<String>,
    /// Leverage for short positions
    #[serde(rename = "shortLeverage", skip_serializing_if = "Option::is_none")]
    pub short_leverage: Option<String>,
    /// Leverage in crossed margin mode
    #[serde(rename = "crossMarginLeverage", skip_serializing_if = "Option::is_none")]
    pub cross_margin_leverage: Option<String>,
    /// Margin mode
    #[serde(rename = "marginMode", skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<MarginMode>,
}

impl LeverageResponse {
    pub fn new() -> LeverageResponse {
        LeverageResponse {
            symbol: None,
            margin_coin: None,
            long_leverage: None,
            short_leverage: None,
            cross_margin_leverage: None,
            margin_mode: None,
        }
    }
}
/// Margin mode
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MarginMode {
    #[serde(rename = "crossed")]
    Crossed,
    #[serde(rename = "isolated")]
    Isolated,
}

impl Default for MarginMode {
    fn default() -> MarginMode {
        Self::Crossed
    }
}

