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
pub struct Trade {
    /// Trading pair, e.g., \"BFTUSDT\"
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Unique identifier for the trade
    #[serde(rename = "tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    /// The side of the trade, either \"buy\" or \"sell\"
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    /// Price at which the trade occurred
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Size of the trade
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Timestamp of the trade in Unix milliseconds
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
}

impl Trade {
    pub fn new() -> Trade {
        Trade {
            symbol: None,
            trade_id: None,
            side: None,
            price: None,
            size: None,
            ts: None,
        }
    }
}
/// The side of the trade, either \"buy\" or \"sell\"
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

impl Default for Side {
    fn default() -> Side {
        Self::Buy
    }
}

