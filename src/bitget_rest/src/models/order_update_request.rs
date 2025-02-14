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
pub struct OrderUpdateRequest {
    /// Trading pair name, e.g., BTCUSDT. All symbols can be returned by the Get Symbol Info interface.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Limit price. The decimal places of price and the price step can be returned by the Get Symbol Info interface.
    #[serde(rename = "price")]
    pub price: String,
    /// Amount, representing the number of base coins.
    #[serde(rename = "size")]
    pub size: String,
    /// Client Order ID. Either orderId or clientOid is required.
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
    /// Order ID. Either orderId or clientOid is required.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// New customized order ID. The idempotency time is 6 hours, only valid when orders are unfilled.
    #[serde(rename = "newClientOid", skip_serializing_if = "Option::is_none")]
    pub new_client_oid: Option<String>,
    /// Take profit price. The decimal places of price and the price step can be returned by the Get Symbol Info interface.
    #[serde(rename = "presetTakeProfitPrice", skip_serializing_if = "Option::is_none")]
    pub preset_take_profit_price: Option<String>,
    /// Take profit execute price. The decimal places of price and the price step can be returned by the Get Symbol Info interface.
    #[serde(rename = "executeTakeProfitPrice", skip_serializing_if = "Option::is_none")]
    pub execute_take_profit_price: Option<String>,
    /// Stop loss price. The decimal places of price and the price step can be returned by the Get Symbol Info interface.
    #[serde(rename = "presetStopLossPrice", skip_serializing_if = "Option::is_none")]
    pub preset_stop_loss_price: Option<String>,
    /// Stop loss execute price. The decimal places of price and the price step can be returned by the Get Symbol Info interface.
    #[serde(rename = "executeStopLossPrice", skip_serializing_if = "Option::is_none")]
    pub execute_stop_loss_price: Option<String>,
}

impl OrderUpdateRequest {
    pub fn new(symbol: String, price: String, size: String) -> OrderUpdateRequest {
        OrderUpdateRequest {
            symbol,
            price,
            size,
            client_oid: None,
            order_id: None,
            new_client_oid: None,
            preset_take_profit_price: None,
            execute_take_profit_price: None,
            preset_stop_loss_price: None,
            execute_stop_loss_price: None,
        }
    }
}

