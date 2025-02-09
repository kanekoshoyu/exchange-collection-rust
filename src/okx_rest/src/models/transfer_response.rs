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
pub struct TransferResponse {
    /// Transfer ID
    #[serde(rename = "transId")]
    pub trans_id: String,
    /// Client-supplied ID
    #[serde(rename = "clientId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<Option<String>>,
    /// Currency
    #[serde(rename = "ccy")]
    pub ccy: String,
    /// The remitting account
    #[serde(rename = "from")]
    pub from: From,
    /// Transfer amount
    #[serde(rename = "amt")]
    pub amt: String,
    /// The beneficiary account
    #[serde(rename = "to")]
    pub to: To,
}

impl TransferResponse {
    pub fn new(trans_id: String, ccy: String, from: From, amt: String, to: To) -> TransferResponse {
        TransferResponse {
            trans_id,
            client_id: None,
            ccy,
            from,
            amt,
            to,
        }
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

