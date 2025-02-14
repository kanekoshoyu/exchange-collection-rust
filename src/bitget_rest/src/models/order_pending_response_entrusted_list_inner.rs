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
pub struct OrderPendingResponseEntrustedListInner {
    /// Trading pair, e.g., ethusdt.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Size of the order.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Unique identifier for the order.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Custom order ID provided by the client.
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
    /// Base volume of the order.
    #[serde(rename = "baseVolume", skip_serializing_if = "Option::is_none")]
    pub base_volume: Option<String>,
    /// Fee associated with the order.
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    /// Price at which the order was placed.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Average price at which the order was filled.
    #[serde(rename = "priceAvg", skip_serializing_if = "Option::is_none")]
    pub price_avg: Option<String>,
    /// Current status of the order, e.g., 'partially_filled'.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Order side, e.g., 'buy' or 'sell'.
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Time-in-force setting for the order, e.g., 'gtc'.
    #[serde(rename = "force", skip_serializing_if = "Option::is_none")]
    pub force: Option<String>,
    /// Total profits from the order.
    #[serde(rename = "totalProfits", skip_serializing_if = "Option::is_none")]
    pub total_profits: Option<String>,
    /// Position side, e.g., 'long' or 'short'.
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
    /// The coin used for margin, e.g., 'usdt'.
    #[serde(rename = "marginCoin", skip_serializing_if = "Option::is_none")]
    pub margin_coin: Option<String>,
    /// Quote volume of the order.
    #[serde(rename = "quoteVolume", skip_serializing_if = "Option::is_none")]
    pub quote_volume: Option<String>,
    /// Leverage applied to the order.
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    /// Margin mode, e.g., 'cross' or 'isolated'.
    #[serde(rename = "marginMode", skip_serializing_if = "Option::is_none")]
    pub margin_mode: Option<String>,
    /// Source of the entry point, e.g., 'api'.
    #[serde(rename = "enterPointSource", skip_serializing_if = "Option::is_none")]
    pub enter_point_source: Option<String>,
    /// Trade side, e.g., 'open' or 'close'.
    #[serde(rename = "tradeSide", skip_serializing_if = "Option::is_none")]
    pub trade_side: Option<String>,
    /// Position mode, e.g., 'hedge_mode'.
    #[serde(rename = "posMode", skip_serializing_if = "Option::is_none")]
    pub pos_mode: Option<String>,
    /// Type of the order, e.g., 'limit' or 'market'.
    #[serde(rename = "orderType", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Source of the order, e.g., 'normal'.
    #[serde(rename = "orderSource", skip_serializing_if = "Option::is_none")]
    pub order_source: Option<String>,
    /// Creation time of the order in milliseconds.
    #[serde(rename = "cTime", skip_serializing_if = "Option::is_none")]
    pub c_time: Option<String>,
    /// Last update time of the order in milliseconds.
    #[serde(rename = "uTime", skip_serializing_if = "Option::is_none")]
    pub u_time: Option<String>,
    /// Preset stop surplus price.
    #[serde(rename = "presetStopSurplusPrice", skip_serializing_if = "Option::is_none")]
    pub preset_stop_surplus_price: Option<String>,
    /// Preset stop loss price.
    #[serde(rename = "presetStopLossPrice", skip_serializing_if = "Option::is_none")]
    pub preset_stop_loss_price: Option<String>,
}

impl OrderPendingResponseEntrustedListInner {
    pub fn new() -> OrderPendingResponseEntrustedListInner {
        OrderPendingResponseEntrustedListInner {
            symbol: None,
            size: None,
            order_id: None,
            client_oid: None,
            base_volume: None,
            fee: None,
            price: None,
            price_avg: None,
            status: None,
            side: None,
            force: None,
            total_profits: None,
            pos_side: None,
            margin_coin: None,
            quote_volume: None,
            leverage: None,
            margin_mode: None,
            enter_point_source: None,
            trade_side: None,
            pos_mode: None,
            order_type: None,
            order_source: None,
            c_time: None,
            u_time: None,
            preset_stop_surplus_price: None,
            preset_stop_loss_price: None,
        }
    }
}

