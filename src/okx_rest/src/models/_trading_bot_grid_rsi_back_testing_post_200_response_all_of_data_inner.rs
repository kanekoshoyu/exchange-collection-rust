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
pub struct TradingBotGridRsiBackTestingPost200ResponseAllOfDataInner {
    #[serde(rename = "triggerNum", skip_serializing_if = "Option::is_none")]
    pub trigger_num: Option<String>,
}

impl TradingBotGridRsiBackTestingPost200ResponseAllOfDataInner {
    pub fn new() -> TradingBotGridRsiBackTestingPost200ResponseAllOfDataInner {
        TradingBotGridRsiBackTestingPost200ResponseAllOfDataInner {
            trigger_num: None,
        }
    }
}

