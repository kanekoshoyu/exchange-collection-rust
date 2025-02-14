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
pub struct Transfer {
    /// Currency
    #[serde(rename = "ccy")]
    pub ccy: String,
    /// Transfer amount
    #[serde(rename = "amt")]
    pub amt: String,
    /// Account type of transfer from sub-account
    #[serde(rename = "from")]
    pub from: From,
    /// Account type of transfer to sub-account
    #[serde(rename = "to")]
    pub to: To,
    /// Sub-account name of the account that transfers funds out
    #[serde(rename = "fromSubAccount")]
    pub from_sub_account: String,
    /// Sub-account name of the account that transfers funds in
    #[serde(rename = "toSubAccount")]
    pub to_sub_account: String,
    /// Whether or not borrowed coins can be transferred out under Multi-currency margin/Portfolio margin
    #[serde(rename = "loanTrans", skip_serializing_if = "Option::is_none")]
    pub loan_trans: Option<bool>,
    /// Ignore position risk (Applicable to Portfolio margin schema)
    #[serde(rename = "omitPosRisk", skip_serializing_if = "Option::is_none")]
    pub omit_pos_risk: Option<OmitPosRisk>,
}

impl Transfer {
    pub fn new(ccy: String, amt: String, from: From, to: To, from_sub_account: String, to_sub_account: String) -> Transfer {
        Transfer {
            ccy,
            amt,
            from,
            to,
            from_sub_account,
            to_sub_account,
            loan_trans: None,
            omit_pos_risk: None,
        }
    }
}
/// Account type of transfer from sub-account
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum From {
    #[serde(rename = "6")]
    Variant6,
    #[serde(rename = "18")]
    Variant18,
}

impl Default for From {
    fn default() -> From {
        Self::Variant6
    }
}
/// Account type of transfer to sub-account
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum To {
    #[serde(rename = "6")]
    Variant6,
    #[serde(rename = "18")]
    Variant18,
}

impl Default for To {
    fn default() -> To {
        Self::Variant6
    }
}
/// Ignore position risk (Applicable to Portfolio margin schema)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OmitPosRisk {
    #[serde(rename = "true")]
    True,
    #[serde(rename = "false")]
    False,
}

impl Default for OmitPosRisk {
    fn default() -> OmitPosRisk {
        Self::True
    }
}

