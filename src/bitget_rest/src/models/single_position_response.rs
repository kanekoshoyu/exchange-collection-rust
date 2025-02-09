/*
 * Bitget API
 *
 * Welcome to Bitget Developer document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SinglePositionResponse {
    /// The margin coin used in the position.
    #[serde(rename = "marginCoin")]
    pub margin_coin: String,
    /// The trading pair symbol.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// The position direction.
    #[serde(rename = "holdSide")]
    pub hold_side: HoldSide,
    /// The size of the open delegate (in terms of position size).
    #[serde(rename = "openDelegateSize")]
    pub open_delegate_size: String,
    /// The total margin size used for the position.
    #[serde(rename = "marginSize")]
    pub margin_size: String,
    /// The available margin for the position.
    #[serde(rename = "available")]
    pub available: String,
    /// The locked margin for the position.
    #[serde(rename = "locked")]
    pub locked: String,
    /// The total margin for the position.
    #[serde(rename = "total")]
    pub total: String,
    /// The leverage applied to the position.
    #[serde(rename = "leverage")]
    pub leverage: String,
    /// The profits achieved from the position.
    #[serde(rename = "achievedProfits", skip_serializing_if = "Option::is_none")]
    pub achieved_profits: Option<String>,
    /// The average open price of the position.
    #[serde(rename = "openPriceAvg")]
    pub open_price_avg: String,
    /// The margin mode (isolated or crossed).
    #[serde(rename = "marginMode")]
    pub margin_mode: MarginMode,
    /// The position mode, either \"hedge_mode\" or \"one_way_mode\".
    #[serde(rename = "posMode")]
    pub pos_mode: PosMode,
    /// The unrealized profit or loss for the position.
    #[serde(rename = "unrealizedPL")]
    pub unrealized_pl: String,
    /// The liquidation price of the position.
    #[serde(rename = "liquidationPrice")]
    pub liquidation_price: String,
    /// The margin rate required to maintain the position.
    #[serde(rename = "keepMarginRate")]
    pub keep_margin_rate: String,
    /// The current market price of the asset.
    #[serde(rename = "markPrice")]
    pub mark_price: String,
    /// The break-even price for the position.
    #[serde(rename = "breakEvenPrice")]
    pub break_even_price: String,
    /// The total fees associated with the position.
    #[serde(rename = "totalFee")]
    pub total_fee: String,
    /// The deducted fees from the total fees.
    #[serde(rename = "deductedFee")]
    pub deducted_fee: String,
    /// The margin ratio for the position.
    #[serde(rename = "marginRatio")]
    pub margin_ratio: String,
    /// The asset mode (e.g., \"single\").
    #[serde(rename = "assetMode")]
    pub asset_mode: String,
    /// The creation time of the position (timestamp in milliseconds).
    #[serde(rename = "cTime")]
    pub c_time: String,
    /// The last update time of the position (timestamp in milliseconds).
    #[serde(rename = "uTime")]
    pub u_time: String,
}

impl SinglePositionResponse {
    pub fn new(margin_coin: String, symbol: String, hold_side: HoldSide, open_delegate_size: String, margin_size: String, available: String, locked: String, total: String, leverage: String, open_price_avg: String, margin_mode: MarginMode, pos_mode: PosMode, unrealized_pl: String, liquidation_price: String, keep_margin_rate: String, mark_price: String, break_even_price: String, total_fee: String, deducted_fee: String, margin_ratio: String, asset_mode: String, c_time: String, u_time: String) -> SinglePositionResponse {
        SinglePositionResponse {
            margin_coin,
            symbol,
            hold_side,
            open_delegate_size,
            margin_size,
            available,
            locked,
            total,
            leverage,
            achieved_profits: None,
            open_price_avg,
            margin_mode,
            pos_mode,
            unrealized_pl,
            liquidation_price,
            keep_margin_rate,
            mark_price,
            break_even_price,
            total_fee,
            deducted_fee,
            margin_ratio,
            asset_mode,
            c_time,
            u_time,
        }
    }
}
/// The position direction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HoldSide {
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "short")]
    Short,
}

impl Default for HoldSide {
    fn default() -> HoldSide {
        Self::Long
    }
}
/// The margin mode (isolated or crossed).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MarginMode {
    #[serde(rename = "isolated")]
    Isolated,
    #[serde(rename = "crossed")]
    Crossed,
}

impl Default for MarginMode {
    fn default() -> MarginMode {
        Self::Isolated
    }
}
/// The position mode, either \"hedge_mode\" or \"one_way_mode\".
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PosMode {
    #[serde(rename = "hedge_mode")]
    HedgeMode,
    #[serde(rename = "one_way_mode")]
    OneWayMode,
}

impl Default for PosMode {
    fn default() -> PosMode {
        Self::HedgeMode
    }
}

