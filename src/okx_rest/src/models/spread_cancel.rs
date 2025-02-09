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
pub struct SpreadCancel {
    /// The time the cancellation is triggered.  `triggerTime=0` means Cancel All After is disabled. 
    #[serde(rename = "triggerTime", skip_serializing_if = "Option::is_none")]
    pub trigger_time: Option<String>,
    /// The time the request is received, in Unix timestamp format in milliseconds. 
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
}

impl SpreadCancel {
    pub fn new() -> SpreadCancel {
        SpreadCancel {
            trigger_time: None,
            ts: None,
        }
    }
}

