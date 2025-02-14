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
pub struct LoanData {
    #[serde(rename = "basic", skip_serializing_if = "Option::is_none")]
    pub basic: Option<Vec<models::LoanDataBasicInner>>,
    #[serde(rename = "vip", skip_serializing_if = "Option::is_none")]
    pub vip: Option<Vec<models::LoanDataVipInner>>,
    #[serde(rename = "regular", skip_serializing_if = "Option::is_none")]
    pub regular: Option<Vec<models::LoanDataRegularInner>>,
}

impl LoanData {
    pub fn new() -> LoanData {
        LoanData {
            basic: None,
            vip: None,
            regular: None,
        }
    }
}

