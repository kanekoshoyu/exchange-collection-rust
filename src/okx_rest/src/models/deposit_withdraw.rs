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
pub struct DepositWithdraw {
    /// Estimated complete time (timezone UTC+8). The format is MM/dd/yyyy, h:mm:ss AM/PM. This is only an approximate estimated time, for reference only. 
    #[serde(rename = "estCompleteTime", skip_serializing_if = "Option::is_none")]
    pub est_complete_time: Option<String>,
    /// The detailed stage and status of the deposit/withdrawal. The message before the colon represents the stage, and the message after the colon represents the ongoing status. 
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Hash record on-chain. For withdrawal, if the txId has already been generated, it will return the value. Otherwise, it will return \"\". 
    #[serde(rename = "txId", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Withdrawal ID. When retrieving deposit status, wdId returns blank \"\". 
    #[serde(rename = "wdId", skip_serializing_if = "Option::is_none")]
    pub wd_id: Option<String>,
}

impl DepositWithdraw {
    pub fn new() -> DepositWithdraw {
        DepositWithdraw {
            est_complete_time: None,
            state: None,
            tx_id: None,
            wd_id: None,
        }
    }
}

