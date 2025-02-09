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
pub struct FutureFillsInfo {
    /// The unique identifier for the trade
    #[serde(rename = "tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    /// The price at which the trade was executed
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// The amount of the asset traded
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// The side of the trade (e.g., 'buy' or 'sell')
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Timestamp of when the trade occurred
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    /// The trading pair (e.g., BTCUSDT)
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

impl FutureFillsInfo {
    pub fn new() -> FutureFillsInfo {
        FutureFillsInfo {
            trade_id: None,
            price: None,
            size: None,
            side: None,
            ts: None,
            symbol: None,
        }
    }
}

