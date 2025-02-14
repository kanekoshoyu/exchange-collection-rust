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
pub struct ContractHistory {
    /// Timestamp, millisecond format of Unix timestamp, e.g. 1597026383085
    #[serde(rename = "ts")]
    pub ts: String,
    /// Open interest in the unit of contracts
    #[serde(rename = "oi")]
    pub oi: String,
    /// Open interest in the unit of crypto
    #[serde(rename = "oiCcy", skip_serializing_if = "Option::is_none")]
    pub oi_ccy: Option<String>,
    /// Open interest in the unit of USD
    #[serde(rename = "oiUsd", skip_serializing_if = "Option::is_none")]
    pub oi_usd: Option<String>,
}

impl ContractHistory {
    pub fn new(ts: String, oi: String) -> ContractHistory {
        ContractHistory {
            ts,
            oi,
            oi_ccy: None,
            oi_usd: None,
        }
    }
}

