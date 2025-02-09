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
pub struct CurrencyPair {
    /// Currency pair, e.g., BTC-USDT
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Base currency, e.g., BTC in BTC-USDT
    #[serde(rename = "baseCcy")]
    pub base_ccy: String,
    /// Maximum amount of base currency
    #[serde(rename = "baseCcyMax", skip_serializing_if = "Option::is_none")]
    pub base_ccy_max: Option<String>,
    /// Minimum amount of base currency
    #[serde(rename = "baseCcyMin", skip_serializing_if = "Option::is_none")]
    pub base_ccy_min: Option<String>,
    /// Quote currency, e.g., USDT in BTC-USDT
    #[serde(rename = "quoteCcy")]
    pub quote_ccy: String,
    /// Maximum amount of quote currency
    #[serde(rename = "quoteCcyMax", skip_serializing_if = "Option::is_none")]
    pub quote_ccy_max: Option<String>,
    /// Minimum amount of quote currency
    #[serde(rename = "quoteCcyMin", skip_serializing_if = "Option::is_none")]
    pub quote_ccy_min: Option<String>,
}

impl CurrencyPair {
    pub fn new(inst_id: String, base_ccy: String, quote_ccy: String) -> CurrencyPair {
        CurrencyPair {
            inst_id,
            base_ccy,
            base_ccy_max: None,
            base_ccy_min: None,
            quote_ccy,
            quote_ccy_max: None,
            quote_ccy_min: None,
        }
    }
}

