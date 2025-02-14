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
pub struct TickerInfo {
    /// Trading pair
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// 24h highest price
    #[serde(rename = "high24h", skip_serializing_if = "Option::is_none")]
    pub high24h: Option<f64>,
    /// 24h open price
    #[serde(rename = "open", skip_serializing_if = "Option::is_none")]
    pub open: Option<f64>,
    /// Latest price
    #[serde(rename = "lastPr", skip_serializing_if = "Option::is_none")]
    pub last_pr: Option<f64>,
    /// 24h lowest price
    #[serde(rename = "low24h", skip_serializing_if = "Option::is_none")]
    pub low24h: Option<f64>,
    /// Trading volume in quote currency
    #[serde(rename = "quoteVolume", skip_serializing_if = "Option::is_none")]
    pub quote_volume: Option<f64>,
    /// Trading volume in base currency
    #[serde(rename = "baseVolume", skip_serializing_if = "Option::is_none")]
    pub base_volume: Option<f64>,
    /// Trading volume in USDT
    #[serde(rename = "usdtVolume", skip_serializing_if = "Option::is_none")]
    pub usdt_volume: Option<f64>,
    /// Bid 1 price
    #[serde(rename = "bidPr", skip_serializing_if = "Option::is_none")]
    pub bid_pr: Option<f64>,
    /// Ask 1 price
    #[serde(rename = "askPr", skip_serializing_if = "Option::is_none")]
    pub ask_pr: Option<f64>,
    /// Buying 1 amount
    #[serde(rename = "bidSz", skip_serializing_if = "Option::is_none")]
    pub bid_sz: Option<f64>,
    /// Selling 1 amount
    #[serde(rename = "askSz", skip_serializing_if = "Option::is_none")]
    pub ask_sz: Option<f64>,
    /// UTC±00:00 Entry price
    #[serde(rename = "openUtc", skip_serializing_if = "Option::is_none")]
    pub open_utc: Option<f64>,
    /// Current time Unix millisecond timestamp
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    /// Change at UTC+0, 0.01 means 1%
    #[serde(rename = "changeUtc24h", skip_serializing_if = "Option::is_none")]
    pub change_utc24h: Option<f64>,
    /// 24-hour change, 0.01 means 1%
    #[serde(rename = "change24h", skip_serializing_if = "Option::is_none")]
    pub change24h: Option<f64>,
}

impl TickerInfo {
    pub fn new() -> TickerInfo {
        TickerInfo {
            symbol: None,
            high24h: None,
            open: None,
            last_pr: None,
            low24h: None,
            quote_volume: None,
            base_volume: None,
            usdt_volume: None,
            bid_pr: None,
            ask_pr: None,
            bid_sz: None,
            ask_sz: None,
            open_utc: None,
            ts: None,
            change_utc24h: None,
            change24h: None,
        }
    }
}

