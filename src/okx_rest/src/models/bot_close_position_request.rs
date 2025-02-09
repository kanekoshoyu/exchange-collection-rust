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
pub struct BotClosePositionRequest {
    /// Algo ID.
    #[serde(rename = "algoId", skip_serializing_if = "Option::is_none")]
    pub algo_id: Option<String>,
    /// Market close all positions or not: - `true`: Market close all positions - `false`: Close part of the position. 
    #[serde(rename = "mktClose", skip_serializing_if = "Option::is_none")]
    pub mkt_close: Option<bool>,
    /// Close position amount, in units of the contract. Required if `mktClose` is `false`. 
    #[serde(rename = "sz", skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    /// Close position price. Required if `mktClose` is `false`. 
    #[serde(rename = "px", skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
}

impl BotClosePositionRequest {
    pub fn new() -> BotClosePositionRequest {
        BotClosePositionRequest {
            algo_id: None,
            mkt_close: None,
            sz: None,
            px: None,
        }
    }
}

