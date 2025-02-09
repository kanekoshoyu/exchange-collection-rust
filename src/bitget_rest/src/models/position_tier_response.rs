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
pub struct PositionTierResponse {
    /// The trading pair symbol, e.g., \"BTCUSDT\".
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The leverage level for the trade.
    #[serde(rename = "level")]
    pub level: String,
    /// The starting unit for the leverage range.
    #[serde(rename = "startUnit")]
    pub start_unit: String,
    /// The ending unit for the leverage range.
    #[serde(rename = "endUnit")]
    pub end_unit: String,
    /// The leverage ratio for the position.
    #[serde(rename = "leverage")]
    pub leverage: String,
    /// The margin rate required to maintain the position.
    #[serde(rename = "keepMarginRate")]
    pub keep_margin_rate: String,
}

impl PositionTierResponse {
    pub fn new(symbol: String, level: String, start_unit: String, end_unit: String, leverage: String, keep_margin_rate: String) -> PositionTierResponse {
        PositionTierResponse {
            symbol,
            level,
            start_unit,
            end_unit,
            leverage,
            keep_margin_rate,
        }
    }
}

