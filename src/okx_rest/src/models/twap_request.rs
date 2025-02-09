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
pub struct TwapRequest {
    /// Price variance by percentage, range between [0.0001 ~ 0.01], e.g., 0.01 represents 1%. Determines the range by percentage when placing small buy orders above the best bid price.
    #[serde(rename = "pxVar", skip_serializing_if = "Option::is_none")]
    pub px_var: Option<String>,
    /// Price variance by constant, should be no less than 0 (no upper limit). Determines the range by constant when placing small buy orders above the best bid price.
    #[serde(rename = "pxSpread", skip_serializing_if = "Option::is_none")]
    pub px_spread: Option<String>,
    /// Average amount to be placed for buy orders when the market price is lower than the limit price, within a certain range above the best bid price.
    #[serde(rename = "szLimit")]
    pub sz_limit: String,
    /// Price limit for buy orders, should be no less than 0 (no upper limit). Represents the limit price when placing small buy orders above the best bid price.
    #[serde(rename = "pxLimit")]
    pub px_limit: String,
    /// Time interval in seconds. Determines the time cycle for placing small buy orders above the best bid price based on the time cycle.
    #[serde(rename = "timeInterval")]
    pub time_interval: String,
}

impl TwapRequest {
    pub fn new(sz_limit: String, px_limit: String, time_interval: String) -> TwapRequest {
        TwapRequest {
            px_var: None,
            px_spread: None,
            sz_limit,
            px_limit,
            time_interval,
        }
    }
}

