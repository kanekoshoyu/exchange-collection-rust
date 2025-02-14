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
pub struct AccountBalance {
    /// Currency (e.g., BTC, ETH)
    #[serde(rename = "ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Total balance in the specified currency
    #[serde(rename = "bal", skip_serializing_if = "Option::is_none")]
    pub bal: Option<String>,
    /// Frozen balance, unable to be used for trading or withdrawal
    #[serde(rename = "frozenBal", skip_serializing_if = "Option::is_none")]
    pub frozen_bal: Option<String>,
    /// Available balance that can be used for trading or withdrawal
    #[serde(rename = "availBal", skip_serializing_if = "Option::is_none")]
    pub avail_bal: Option<String>,
}

impl AccountBalance {
    pub fn new() -> AccountBalance {
        AccountBalance {
            ccy: None,
            bal: None,
            frozen_bal: None,
            avail_bal: None,
        }
    }
}

