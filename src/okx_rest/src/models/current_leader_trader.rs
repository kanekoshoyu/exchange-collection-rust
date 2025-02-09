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
pub struct CurrentLeaderTrader {
    /// Portrait link of the trader.
    #[serde(rename = "portLink")]
    pub port_link: String,
    /// Nickname of the trader.
    #[serde(rename = "nickName")]
    pub nick_name: String,
    /// Margin for copy trading.
    #[serde(rename = "margin")]
    pub margin: String,
    /// Total amount to copy in USDT.
    #[serde(rename = "copyTotalAmt")]
    pub copy_total_amt: String,
    /// Total profit and loss for the copy trading.
    #[serde(rename = "copyTotalPnl")]
    pub copy_total_pnl: String,
    /// Lead trader's unique code.
    #[serde(rename = "uniqueCode")]
    pub unique_code: String,
    /// Margin currency.
    #[serde(rename = "ccy")]
    pub ccy: String,
    /// Profit sharing ratio, e.g., \"0.1\" represents 10%.
    #[serde(rename = "profitSharingRatio")]
    pub profit_sharing_ratio: String,
    /// Time when copying started, in Unix timestamp format (milliseconds).
    #[serde(rename = "beginCopyTime")]
    pub begin_copy_time: String,
    /// Unrealized profit and loss.
    #[serde(rename = "upl")]
    pub upl: String,
    /// Profit and loss for today.
    #[serde(rename = "todayPnl")]
    pub today_pnl: String,
    /// Mode of the lead trader.
    #[serde(rename = "leadMode")]
    pub lead_mode: LeadMode,
}

impl CurrentLeaderTrader {
    pub fn new(port_link: String, nick_name: String, margin: String, copy_total_amt: String, copy_total_pnl: String, unique_code: String, ccy: String, profit_sharing_ratio: String, begin_copy_time: String, upl: String, today_pnl: String, lead_mode: LeadMode) -> CurrentLeaderTrader {
        CurrentLeaderTrader {
            port_link,
            nick_name,
            margin,
            copy_total_amt,
            copy_total_pnl,
            unique_code,
            ccy,
            profit_sharing_ratio,
            begin_copy_time,
            upl,
            today_pnl,
            lead_mode,
        }
    }
}
/// Mode of the lead trader.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LeadMode {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl Default for LeadMode {
    fn default() -> LeadMode {
        Self::Public
    }
}

