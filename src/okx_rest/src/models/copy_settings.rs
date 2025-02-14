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
pub struct CopySettings {
    /// Copy mode.
    #[serde(rename = "copyMode")]
    pub copy_mode: CopyMode,
    /// Copy amount in USDT per order. Required if `copyMode` is `fixed_amount`.
    #[serde(rename = "copyAmt", skip_serializing_if = "Option::is_none")]
    pub copy_amt: Option<String>,
    /// Copy ratio per order. Required if `copyMode` is `ratio_copy`.
    #[serde(rename = "copyRatio", skip_serializing_if = "Option::is_none")]
    pub copy_ratio: Option<String>,
    /// Maximum total amount in USDT you'll invest across all orders in this copy trade.
    #[serde(rename = "copyTotalAmt")]
    pub copy_total_amt: String,
    /// Take profit per order. 0.1 represents 10%.
    #[serde(rename = "tpRatio", skip_serializing_if = "Option::is_none")]
    pub tp_ratio: Option<String>,
    /// Stop loss per order. 0.1 represents 10%.
    #[serde(rename = "slRatio", skip_serializing_if = "Option::is_none")]
    pub sl_ratio: Option<String>,
    /// Copy contract type.
    #[serde(rename = "copyInstIdType")]
    pub copy_inst_id_type: CopyInstIdType,
    /// Instrument list. Includes all lead contracts of the lead trader.
    #[serde(rename = "instIds", skip_serializing_if = "Option::is_none")]
    pub inst_ids: Option<Vec<models::CopySettingsInstIdsInner>>,
    /// Total stop loss in USDT for the trader. Stop copying when net loss reaches this value.
    #[serde(rename = "slTotalAmt", skip_serializing_if = "Option::is_none")]
    pub sl_total_amt: Option<String>,
    /// Action type for open positions. Default is `copy_close`.
    #[serde(rename = "subPosCloseType", skip_serializing_if = "Option::is_none")]
    pub sub_pos_close_type: Option<SubPosCloseType>,
    /// Copy margin mode.
    #[serde(rename = "copyMgnMode")]
    pub copy_mgn_mode: CopyMgnMode,
    /// Margin currency.
    #[serde(rename = "ccy")]
    pub ccy: String,
    /// Current copy state.
    #[serde(rename = "copyState", skip_serializing_if = "Option::is_none")]
    pub copy_state: Option<CopyState>,
}

impl CopySettings {
    pub fn new(copy_mode: CopyMode, copy_total_amt: String, copy_inst_id_type: CopyInstIdType, copy_mgn_mode: CopyMgnMode, ccy: String) -> CopySettings {
        CopySettings {
            copy_mode,
            copy_amt: None,
            copy_ratio: None,
            copy_total_amt,
            tp_ratio: None,
            sl_ratio: None,
            copy_inst_id_type,
            inst_ids: None,
            sl_total_amt: None,
            sub_pos_close_type: None,
            copy_mgn_mode,
            ccy,
            copy_state: None,
        }
    }
}
/// Copy mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CopyMode {
    #[serde(rename = "fixed_amount")]
    FixedAmount,
    #[serde(rename = "ratio_copy")]
    RatioCopy,
}

impl Default for CopyMode {
    fn default() -> CopyMode {
        Self::FixedAmount
    }
}
/// Copy contract type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CopyInstIdType {
    #[serde(rename = "custom")]
    Custom,
    #[serde(rename = "copy")]
    Copy,
}

impl Default for CopyInstIdType {
    fn default() -> CopyInstIdType {
        Self::Custom
    }
}
/// Action type for open positions. Default is `copy_close`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubPosCloseType {
    #[serde(rename = "market_close")]
    MarketClose,
    #[serde(rename = "copy_close")]
    CopyClose,
    #[serde(rename = "manual_close")]
    ManualClose,
}

impl Default for SubPosCloseType {
    fn default() -> SubPosCloseType {
        Self::MarketClose
    }
}
/// Copy margin mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CopyMgnMode {
    #[serde(rename = "cross")]
    Cross,
    #[serde(rename = "isolated")]
    Isolated,
    #[serde(rename = "copy")]
    Copy,
}

impl Default for CopyMgnMode {
    fn default() -> CopyMgnMode {
        Self::Cross
    }
}
/// Current copy state.
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

