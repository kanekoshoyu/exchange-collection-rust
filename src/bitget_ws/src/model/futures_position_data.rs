#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FuturesPositionData represents a FuturesPositionData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FuturesPositionData {
    #[serde(rename="posId")]
    pub pos_id: String,
    #[serde(rename="instId")]
    pub inst_id: String,
    #[serde(rename="marginCoin")]
    pub margin_coin: String,
    #[serde(rename="marginSize")]
    pub margin_size: String,
    #[serde(rename="marginMode")]
    pub margin_mode: String,
    #[serde(rename="holdSide")]
    pub hold_side: String,
    #[serde(rename="posMode")]
    pub pos_mode: String,
    #[serde(rename="total")]
    pub total: String,
    #[serde(rename="available")]
    pub available: String,
    #[serde(rename="frozen")]
    pub frozen: String,
    #[serde(rename="openPriceAvg")]
    pub open_price_avg: String,
    #[serde(rename="leverage")]
    pub leverage: i32,
    #[serde(rename="achievedProfits")]
    pub achieved_profits: String,
    #[serde(rename="unrealizedPL")]
    pub unrealized_pl: String,
    #[serde(rename="unrealizedPLR")]
    pub unrealized_plr: String,
    #[serde(rename="liquidationPrice")]
    pub liquidation_price: String,
    #[serde(rename="keepMarginRate")]
    pub keep_margin_rate: String,
    #[serde(rename="marginRate")]
    pub margin_rate: String,
    #[serde(rename="cTime")]
    pub c_time: String,
    #[serde(rename="breakEvenPrice")]
    pub break_even_price: String,
    #[serde(rename="totalFee")]
    pub total_fee: String,
    #[serde(rename="deductedFee")]
    pub deducted_fee: String,
    #[serde(rename="uTime")]
    pub u_time: String,
    #[serde(rename="autoMargin")]
    pub auto_margin: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

