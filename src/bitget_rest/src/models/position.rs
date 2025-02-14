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
pub struct Position {
    /// Unique identifier for the position.
    #[serde(rename = "positionId", skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// The margin coin used in the position.
    #[serde(rename = "marginCoin", skip_serializing_if = "Option::is_none")]
    pub margin_coin: Option<String>,
    /// The trading pair symbol.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// The position direction (e.g., long or short).
    #[serde(rename = "holdSide", skip_serializing_if = "Option::is_none")]
    pub hold_side: Option<String>,
    /// The average price at which the position was opened.
    #[serde(rename = "openAvgPrice", skip_serializing_if = "Option::is_none")]
    pub open_avg_price: Option<String>,
    /// The average price at which the position was closed.
    #[serde(rename = "closeAvgPrice", skip_serializing_if = "Option::is_none")]
    pub close_avg_price: Option<String>,
    /// The margin mode for the position (isolated or crossed).
    #[serde(rename = "marginMode", skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    /// The total position size when the position was opened.
    #[serde(rename = "openTotalPos", skip_serializing_if = "Option::is_none")]
    pub open_total_pos: Option<String>,
    /// The total position size when the position was closed.
    #[serde(rename = "closeTotalPos", skip_serializing_if = "Option::is_none")]
    pub close_total_pos: Option<String>,
    /// The profit and loss from the position.
    #[serde(rename = "pnl", skip_serializing_if = "Option::is_none")]
    pub pnl: Option<String>,
    /// The net profit from the position after all costs.
    #[serde(rename = "netProfit", skip_serializing_if = "Option::is_none")]
    pub net_profit: Option<String>,
    /// The total funding fee for the position.
    #[serde(rename = "totalFunding", skip_serializing_if = "Option::is_none")]
    pub total_funding: Option<String>,
    /// The fee when opening the position.
    #[serde(rename = "openFee", skip_serializing_if = "Option::is_none")]
    pub open_fee: Option<String>,
    /// The fee when closing the position.
    #[serde(rename = "closeFee", skip_serializing_if = "Option::is_none")]
    pub close_fee: Option<String>,
    /// The timestamp when the position was created.
    #[serde(rename = "cTime", skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    /// The timestamp when the position was last updated.
    #[serde(rename = "uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
}

impl Position {
    pub fn new() -> Position {
        Position {
            position_id: None,
            margin_coin: None,
            symbol: None,
            hold_side: None,
            open_avg_price: None,
            close_avg_price: None,
            margin_mode: None,
            open_total_pos: None,
            close_total_pos: None,
            pnl: None,
            net_profit: None,
            total_funding: None,
            open_fee: None,
            close_fee: None,
            c_time: None,
            u_time: None,
        }
    }
}

