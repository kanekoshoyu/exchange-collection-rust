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
pub struct PrefStats {
    /// Win ratio (e.g., 0.1 represents 10%)
    #[serde(rename = "ratio", skip_serializing_if = "Option::is_none")]
    pub ratio: Option<String>,
    /// Margin currency
    #[serde(rename = "ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
}

impl PrefStats {
    pub fn new() -> PrefStats {
        PrefStats {
            ratio: None,
            ccy: None,
        }
    }
}

