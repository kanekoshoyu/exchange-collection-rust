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
pub struct SubPosition {
    /// Instrument ID (e.g., BTC-USDT-SWAP)
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// Lead position ID
    #[serde(rename = "subPosId")]
    pub sub_pos_id: String,
    /// Position side (long, short, or net)
    #[serde(rename = "posSide")]
    pub pos_side: PosSide,
    /// Margin mode (cross or isolated)
    #[serde(rename = "mgnMode")]
    pub mgn_mode: MgnMode,
    /// Leverage
    #[serde(rename = "lever")]
    pub lever: String,
    /// Average open price
    #[serde(rename = "openAvgPx")]
    pub open_avg_px: String,
    /// Open time (e.g., Unix timestamp)
    #[serde(rename = "openTime")]
    pub open_time: String,
    /// Quantity of positions
    #[serde(rename = "subPos")]
    pub sub_pos: String,
    /// Instrument type
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// Margin amount
    #[serde(rename = "margin")]
    pub margin: String,
    /// Unrealized profit and loss
    #[serde(rename = "upl")]
    pub upl: String,
    /// Unrealized profit and loss ratio
    #[serde(rename = "uplRatio")]
    pub upl_ratio: String,
    /// Latest mark price (only applicable to contract)
    #[serde(rename = "markPx")]
    pub mark_px: String,
    /// Lead trader unique code
    #[serde(rename = "uniqueCode")]
    pub unique_code: String,
    /// Currency (e.g., USDT)
    #[serde(rename = "ccy")]
    pub ccy: String,
}

impl SubPosition {
    pub fn new(inst_id: String, sub_pos_id: String, pos_side: PosSide, mgn_mode: MgnMode, lever: String, open_avg_px: String, open_time: String, sub_pos: String, inst_type: String, margin: String, upl: String, upl_ratio: String, mark_px: String, unique_code: String, ccy: String) -> SubPosition {
        SubPosition {
            inst_id,
            sub_pos_id,
            pos_side,
            mgn_mode,
            lever,
            open_avg_px,
            open_time,
            sub_pos,
            inst_type,
            margin,
            upl,
            upl_ratio,
            mark_px,
            unique_code,
            ccy,
        }
    }
}
/// Position side (long, short, or net)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PosSide {
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "net")]
    Net,
}

impl Default for PosSide {
    fn default() -> PosSide {
        Self::Long
    }
}
/// Margin mode (cross or isolated)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MgnMode {
    #[serde(rename = "cross")]
    Cross,
    #[serde(rename = "isolated")]
    Isolated,
}

impl Default for MgnMode {
    fn default() -> MgnMode {
        Self::Cross
    }
}

