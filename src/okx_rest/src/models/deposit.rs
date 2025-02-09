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
pub struct Deposit {
    /// Deposit address
    #[serde(rename = "addr", skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    /// Deposit tag (will not be returned if the currency does not require a tag for deposit) 
    #[serde(rename = "tag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tag: Option<Option<String>>,
    /// Deposit memo (will not be returned if the currency does not require a memo for deposit) 
    #[serde(rename = "memo", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub memo: Option<Option<String>>,
    /// Deposit payment ID (will not be returned if the currency does not require a payment_id for deposit) 
    #[serde(rename = "pmtId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub pmt_id: Option<Option<String>>,
    /// Deposit address attachment (will not be returned if the currency does not require this).  Example: For TONCOIN, the attached tag name is 'comment', e.g., {'comment':'123456'} 
    #[serde(rename = "addrEx", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub addr_ex: Option<Option<std::collections::HashMap<String, String>>>,
    /// Currency, e.g., BTC
    #[serde(rename = "ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Chain name, e.g., USDT-ERC20, USDT-TRC20
    #[serde(rename = "chain", skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// The beneficiary account. 6: Funding account 18: Trading account Note: Some users (e.g., Brazil) only support deposits to the trading account. 
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<To>,
    /// Verified name (for recipient)
    #[serde(rename = "verifiedName", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<Option<String>>,
    /// Indicates if the current deposit address is selected by the website page
    #[serde(rename = "selected", skip_serializing_if = "Option::is_none")]
    pub selected: Option<bool>,
    /// Last 6 digits of the contract address
    #[serde(rename = "ctAddr", skip_serializing_if = "Option::is_none")]
    pub ct_addr: Option<String>,
}

impl Deposit {
    pub fn new() -> Deposit {
        Deposit {
            addr: None,
            tag: None,
            memo: None,
            pmt_id: None,
            addr_ex: None,
            ccy: None,
            chain: None,
            to: None,
            verified_name: None,
            selected: None,
            ct_addr: None,
        }
    }
}
/// The beneficiary account. 6: Funding account 18: Trading account Note: Some users (e.g., Brazil) only support deposits to the trading account. 
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

