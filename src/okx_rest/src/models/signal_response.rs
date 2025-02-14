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
pub struct SignalResponse {
    /// The ID of the signal channel.
    #[serde(rename = "signalChanId", skip_serializing_if = "Option::is_none")]
    pub signal_chan_id: Option<String>,
    /// User identify when placing orders via signal.
    #[serde(rename = "signalChanToken", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub signal_chan_token: Option<Option<String>>,
}

impl SignalResponse {
    pub fn new() -> SignalResponse {
        SignalResponse {
            signal_chan_id: None,
            signal_chan_token: None,
        }
    }
}

