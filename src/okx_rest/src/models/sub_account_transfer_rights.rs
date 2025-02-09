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
pub struct SubAccountTransferRights {
    /// Name of the sub-account(s). Single sub-account or multiple sub-accounts (no more than 20), separated by commas.
    #[serde(rename = "subAcct")]
    pub sub_acct: String,
    /// Whether the sub-account has the right to transfer out. The default is true.
    #[serde(rename = "canTransOut", skip_serializing_if = "Option::is_none")]
    pub can_trans_out: Option<bool>,
}

impl SubAccountTransferRights {
    pub fn new(sub_acct: String) -> SubAccountTransferRights {
        SubAccountTransferRights {
            sub_acct,
            can_trans_out: None,
        }
    }
}

