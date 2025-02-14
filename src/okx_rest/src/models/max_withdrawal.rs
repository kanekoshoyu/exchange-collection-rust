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
pub struct MaxWithdrawal {
    /// Currency (e.g., BTC, ETH)
    #[serde(rename = "ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Max withdrawal amount (excluding borrowed assets under Multi-currency margin)
    #[serde(rename = "maxWd", skip_serializing_if = "Option::is_none")]
    pub max_wd: Option<String>,
    /// Max withdrawal amount (including borrowed assets under Multi-currency margin)
    #[serde(rename = "maxWdEx", skip_serializing_if = "Option::is_none")]
    pub max_wd_ex: Option<String>,
    /// Max withdrawal under Spot-Derivatives risk offset mode (excluding borrowed assets under Portfolio margin)
    #[serde(rename = "spotOffsetMaxWd", skip_serializing_if = "Option::is_none")]
    pub spot_offset_max_wd: Option<String>,
    /// Max withdrawal under Spot-Derivatives risk offset mode (including borrowed assets under Portfolio margin)
    #[serde(rename = "spotOffsetMaxWdEx", skip_serializing_if = "Option::is_none")]
    pub spot_offset_max_wd_ex: Option<String>,
}

impl MaxWithdrawal {
    pub fn new() -> MaxWithdrawal {
        MaxWithdrawal {
            ccy: None,
            max_wd: None,
            max_wd_ex: None,
            spot_offset_max_wd: None,
            spot_offset_max_wd_ex: None,
        }
    }
}

