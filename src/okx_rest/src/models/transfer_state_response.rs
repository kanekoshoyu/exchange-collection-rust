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
pub struct TransferStateResponse {
    /// Transfer ID
    #[serde(rename = "transId", skip_serializing_if = "Option::is_none")]
    pub trans_id: Option<String>,
    /// Client-supplied ID
    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// Currency to be transferred, e.g., USDT
    #[serde(rename = "ccy")]
    pub ccy: String,
    /// Amount to be transferred
    #[serde(rename = "amt")]
    pub amt: String,
    /// Transfer type
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The remitting account
    #[serde(rename = "from")]
    pub from: From,
    /// The beneficiary account
    #[serde(rename = "to")]
    pub to: To,
    /// Name of the sub-account (Required for transfer types 1, 2, 3, and 4)
    #[serde(rename = "subAcct", skip_serializing_if = "Option::is_none")]
    pub sub_acct: Option<String>,
    /// Deprecated field
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// Deprecated field
    #[serde(rename = "toInstId", skip_serializing_if = "Option::is_none")]
    pub to_inst_id: Option<String>,
    /// Transfer state
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl TransferStateResponse {
    pub fn new(ccy: String, amt: String, r#type: Type, from: From, to: To) -> TransferStateResponse {
        TransferStateResponse {
            trans_id: None,
            client_id: None,
            ccy,
            amt,
            r#type,
            from,
            to,
            sub_acct: None,
            inst_id: None,
            to_inst_id: None,
            state: None,
        }
    }
}
/// Transfer type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "3")]
    Variant3,
    #[serde(rename = "4")]
    Variant4,
}

impl Default for Type {
    fn default() -> Type {
        Self::Variant0
    }
}
/// The remitting account
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
/// The beneficiary account
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
/// Transfer state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for State {
    fn default() -> State {
        Self::Success
    }
}

