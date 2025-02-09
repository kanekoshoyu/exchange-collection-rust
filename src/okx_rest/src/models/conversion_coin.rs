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
pub struct ConversionCoin {
    /// Convert type
    #[serde(rename = "type")]
    pub r#type: Type,
    /// Instrument ID. Only applicable to FUTURES/SWAP/OPTION.
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Order price. Required for crypto-margined contracts, and when converting between USDT and contract.
    #[serde(rename = "px", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub px: Option<Option<String>>,
    /// Quantity to buy or sell. It is the quantity of contract while converting currency to contract, and the quantity of currency while converting contract to currency.
    #[serde(rename = "sz")]
    pub sz: String,
    /// The unit of currency. Applicable to USDⓈ-margined contracts from FUTURES/SWAP.
    #[serde(rename = "unit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unit: Option<Option<Unit>>,
}

impl ConversionCoin {
    pub fn new(r#type: Type, inst_id: String, sz: String) -> ConversionCoin {
        ConversionCoin {
            r#type,
            inst_id,
            px: None,
            sz,
            unit: None,
        }
    }
}
/// Convert type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}

impl Default for Type {
    fn default() -> Type {
        Self::Variant1
    }
}
/// The unit of currency. Applicable to USDⓈ-margined contracts from FUTURES/SWAP.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Unit {
    #[serde(rename = "coin")]
    Coin,
    #[serde(rename = "usds")]
    Usds,
}

impl Default for Unit {
    fn default() -> Unit {
        Self::Coin
    }
}

