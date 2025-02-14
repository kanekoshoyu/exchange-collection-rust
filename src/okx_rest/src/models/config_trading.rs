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
pub struct ConfigTrading {
    /// User unique code
    #[serde(rename = "uniqueCode")]
    pub unique_code: String,
    /// Nickname
    #[serde(rename = "nickName")]
    pub nick_name: String,
    /// Portrait link
    #[serde(rename = "portLink", skip_serializing_if = "Option::is_none")]
    pub port_link: Option<String>,
    /// Details
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: InstType,
    /// Role type
    #[serde(rename = "roleType")]
    pub role_type: RoleType,
    /// Profit sharing ratio. Only applicable to lead trader, or it will be \"\". 0.1 represents 10%.
    #[serde(rename = "profitSharingRatio", skip_serializing_if = "Option::is_none")]
    pub profit_sharing_ratio: Option<String>,
    /// Maximum number of copy traders
    #[serde(rename = "maxCopyTraderNum", skip_serializing_if = "Option::is_none")]
    pub max_copy_trader_num: Option<i32>,
    /// Current number of copy traders
    #[serde(rename = "copyTraderNum", skip_serializing_if = "Option::is_none")]
    pub copy_trader_num: Option<i32>,
}

impl ConfigTrading {
    pub fn new(unique_code: String, nick_name: String, inst_type: InstType, role_type: RoleType) -> ConfigTrading {
        ConfigTrading {
            unique_code,
            nick_name,
            port_link: None,
            details: None,
            inst_type,
            role_type,
            profit_sharing_ratio: None,
            max_copy_trader_num: None,
            copy_trader_num: None,
        }
    }
}
/// Instrument type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstType {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "SWAP")]
    Swap,
}

impl Default for InstType {
    fn default() -> InstType {
        Self::Spot
    }
}
/// Role type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RoleType {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}

impl Default for RoleType {
    fn default() -> RoleType {
        Self::Variant0
    }
}

