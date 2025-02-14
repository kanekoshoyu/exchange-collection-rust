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
pub struct SpreadTradeInfo {
    /// Spread ID
    #[serde(rename = "sprdId", skip_serializing_if = "Option::is_none")]
    pub sprd_id: Option<String>,
    /// Trade ID
    #[serde(rename = "tradeId", skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<String>,
    /// Order ID
    #[serde(rename = "ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    /// Client Order ID as assigned by the client
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    /// Order tag
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Filled price
    #[serde(rename = "fillPx", skip_serializing_if = "Option::is_none")]
    pub fill_px: Option<String>,
    /// Filled quantity
    #[serde(rename = "fillSz", skip_serializing_if = "Option::is_none")]
    pub fill_sz: Option<String>,
    /// Order side
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    /// Trade state
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Liquidity taker or maker
    #[serde(rename = "execType", skip_serializing_if = "Option::is_none")]
    pub exec_type: Option<ExecType>,
    /// Data generation time, Unix timestamp format in milliseconds
    #[serde(rename = "ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    /// Legs of trade
    #[serde(rename = "legs", skip_serializing_if = "Option::is_none")]
    pub legs: Option<Vec<models::SpreadTradeInfoLegsInner>>,
    /// Error Code, the default is 0
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Error Message, the default is \"\"
    #[serde(rename = "msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
}

impl SpreadTradeInfo {
    pub fn new() -> SpreadTradeInfo {
        SpreadTradeInfo {
            sprd_id: None,
            trade_id: None,
            ord_id: None,
            cl_ord_id: None,
            tag: None,
            fill_px: None,
            fill_sz: None,
            side: None,
            state: None,
            exec_type: None,
            ts: None,
            legs: None,
            code: None,
            msg: None,
        }
    }
}
/// Order side
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

impl Default for Side {
    fn default() -> Side {
        Self::Buy
    }
}
/// Trade state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "rejected")]
    Rejected,
}

impl Default for State {
    fn default() -> State {
        Self::Filled
    }
}
/// Liquidity taker or maker
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExecType {
    #[serde(rename = "T")]
    T,
    #[serde(rename = "M")]
    M,
}

impl Default for ExecType {
    fn default() -> ExecType {
        Self::T
    }
}

