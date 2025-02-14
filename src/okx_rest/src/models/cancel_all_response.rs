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
pub struct CancelAllResponse {
    /// The time the cancellation is triggered. triggerTime=0 means Cancel All After is disabled.
    #[serde(rename = "triggerTime")]
    pub trigger_time: String,
    /// CAA order tag. A combination of case-sensitive alphanumerics, all numbers, or all letters of up to 16 characters.
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// The time the request is received.
    #[serde(rename = "ts")]
    pub ts: String,
}

impl CancelAllResponse {
    pub fn new(trigger_time: String, ts: String) -> CancelAllResponse {
        CancelAllResponse {
            trigger_time,
            tag: None,
            ts,
        }
    }
}

