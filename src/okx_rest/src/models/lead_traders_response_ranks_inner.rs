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
pub struct LeadTradersResponseRanksInner {
    /// Assets under management in USDT
    #[serde(rename = "aum", skip_serializing_if = "Option::is_none")]
    pub aum: Option<String>,
    /// Current copy state
    #[serde(rename = "copyState", skip_serializing_if = "Option::is_none")]
    pub copy_state: Option<CopyState>,
    /// Maximum number of copy traders
    #[serde(rename = "maxCopyTraderNum", skip_serializing_if = "Option::is_none")]
    pub max_copy_trader_num: Option<String>,
    /// Current number of copy traders
    #[serde(rename = "copyTraderNum", skip_serializing_if = "Option::is_none")]
    pub copy_trader_num: Option<String>,
    /// Accumulated number of copy traders
    #[serde(rename = "accCopyTraderNum", skip_serializing_if = "Option::is_none")]
    pub acc_copy_trader_num: Option<String>,
    /// Portrait link of the lead trader
    #[serde(rename = "portLink", skip_serializing_if = "Option::is_none")]
    pub port_link: Option<String>,
    /// Nickname of the lead trader
    #[serde(rename = "nickName", skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    /// Margin currency
    #[serde(rename = "ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    /// Lead trader unique code
    #[serde(rename = "uniqueCode", skip_serializing_if = "Option::is_none")]
    pub unique_code: Option<String>,
    /// Win ratio (e.g., 0.1 represents 10%)
    #[serde(rename = "winRatio", skip_serializing_if = "Option::is_none")]
    pub win_ratio: Option<String>,
    /// Number of days the trader has been leading
    #[serde(rename = "leadDays", skip_serializing_if = "Option::is_none")]
    pub lead_days: Option<String>,
    #[serde(rename = "traderInsts", skip_serializing_if = "Option::is_none")]
    pub trader_insts: Option<Vec<String>>,
    /// Profit and loss (P&L) in USDT for the last 90 days
    #[serde(rename = "pnl", skip_serializing_if = "Option::is_none")]
    pub pnl: Option<String>,
    /// Profit and loss ratio over the last 90 days (e.g., 0.1 represents 10%)
    #[serde(rename = "pnlRatio", skip_serializing_if = "Option::is_none")]
    pub pnl_ratio: Option<String>,
    #[serde(rename = "pnlRatios", skip_serializing_if = "Option::is_none")]
    pub pnl_ratios: Option<Vec<models::LeadTradersResponseRanksInnerPnlRatiosInner>>,
}

impl LeadTradersResponseRanksInner {
    pub fn new() -> LeadTradersResponseRanksInner {
        LeadTradersResponseRanksInner {
            aum: None,
            copy_state: None,
            max_copy_trader_num: None,
            copy_trader_num: None,
            acc_copy_trader_num: None,
            port_link: None,
            nick_name: None,
            ccy: None,
            unique_code: None,
            win_ratio: None,
            lead_days: None,
            trader_insts: None,
            pnl: None,
            pnl_ratio: None,
            pnl_ratios: None,
        }
    }
}
/// Current copy state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CopyState {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for CopyState {
    fn default() -> CopyState {
        Self::Variant0
    }
}

