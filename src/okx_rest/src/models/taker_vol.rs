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
pub struct TakerVol {
    /// Timestamp in Unix timestamp format.
    #[serde(rename = "ts")]
    pub ts: String,
    /// Sell volume.
    #[serde(rename = "sellVol")]
    pub sell_vol: String,
    /// Buy volume.
    #[serde(rename = "buyVol")]
    pub buy_vol: String,
}

impl TakerVol {
    pub fn new(ts: String, sell_vol: String, buy_vol: String) -> TakerVol {
        TakerVol {
            ts,
            sell_vol,
            buy_vol,
        }
    }
}

