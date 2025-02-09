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
pub struct AlgoOrderRequest {
    /// Algorithmic Order ID.
    #[serde(rename = "algoId")]
    pub algo_id: String,
    /// Instrument ID, e.g. BTC-USDT.
    #[serde(rename = "instId")]
    pub inst_id: String,
}

impl AlgoOrderRequest {
    pub fn new(algo_id: String, inst_id: String) -> AlgoOrderRequest {
        AlgoOrderRequest {
            algo_id,
            inst_id,
        }
    }
}

