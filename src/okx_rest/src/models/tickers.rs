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
pub struct Tickers {
    /// Instrument type
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// Instrument ID
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Last traded price
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    /// Last traded size. 0 represents there is no trading volume
    #[serde(rename = "lastSz", skip_serializing_if = "Option::is_none")]
    pub last_sz: Option<String>,
    /// Best ask price
    #[serde(rename = "askPx", skip_serializing_if = "Option::is_none")]
    pub ask_px: Option<String>,
    /// Best ask size
    #[serde(rename = "askSz", skip_serializing_if = "Option::is_none")]
    pub ask_sz: Option<String>,
    /// Best bid price
    #[serde(rename = "bidPx", skip_serializing_if = "Option::is_none")]
    pub bid_px: Option<String>,
    /// Best bid size
    #[serde(rename = "bidSz", skip_serializing_if = "Option::is_none")]
    pub bid_sz: Option<String>,
    /// Open price in the past 24 hours
    #[serde(rename = "open24h", skip_serializing_if = "Option::is_none")]
    pub open24h: Option<String>,
    /// Highest price in the past 24 hours
    #[serde(rename = "high24h", skip_serializing_if = "Option::is_none")]
    pub high24h: Option<String>,
    /// Lowest price in the past 24 hours
    #[serde(rename = "low24h", skip_serializing_if = "Option::is_none")]
    pub low24h: Option<String>,
    /// 24h trading volume, with a unit of currency. - For derivatives contracts: The value is the number of base currency. - For SPOT/MARGIN: The value is the quantity in quote currency. 
    #[serde(rename = "volCcy24h", skip_serializing_if = "Option::is_none")]
    pub vol_ccy24h: Option<String>,
    /// 24h trading volume, with a unit of contract. - For derivatives contracts: The value is the number of contracts. - For SPOT/MARGIN: The value is the quantity in base currency. 
    #[serde(rename = "vol24h", skip_serializing_if = "Option::is_none")]
    pub vol24h: Option<String>,
    /// Open price in the UTC 0 timezone
    #[serde(rename = "sodUtc0", skip_serializing_if = "Option::is_none")]
    pub sod_utc0: Option<String>,
    /// Open price in the UTC 8 timezone
    #[serde(rename = "sodUtc8", skip_serializing_if = "Option::is_none")]
    pub sod_utc8: Option<String>,
    /// Ticker data generation time, in Unix timestamp format (milliseconds)
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
}

impl Tickers {
    pub fn new() -> Tickers {
        Tickers {
            inst_type: None,
            inst_id: None,
            last: None,
            last_sz: None,
            ask_px: None,
            ask_sz: None,
            bid_px: None,
            bid_sz: None,
            open24h: None,
            high24h: None,
            low24h: None,
            vol_ccy24h: None,
            vol24h: None,
            sod_utc0: None,
            sod_utc8: None,
            ts: None,
        }
    }
}

