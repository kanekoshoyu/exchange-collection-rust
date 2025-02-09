/*
 * Hashkey Exchange
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpotOrderResponse {
    /// Account number
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    /// Trading pair symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// An ID defined by the client for the order
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// System-generated order ID
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// Timestamp in milliseconds
    #[serde(rename = "transactTime", skip_serializing_if = "Option::is_none")]
    pub transact_time: Option<i64>,
    /// Price
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    /// Quantity
    #[serde(rename = "origQty", skip_serializing_if = "Option::is_none")]
    pub orig_qty: Option<f64>,
    /// Traded Volume
    #[serde(rename = "executedQty", skip_serializing_if = "Option::is_none")]
    pub executed_qty: Option<f64>,
    /// Order status
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Duration of the order before expiring
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    /// Order type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Buy or Sell
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Requested Cash amount
    #[serde(rename = "reqAmount", skip_serializing_if = "Option::is_none")]
    pub req_amount: Option<String>,
    /// Concentration reminder message
    #[serde(rename = "concentration", skip_serializing_if = "Option::is_none")]
    pub concentration: Option<String>,
}

impl SpotOrderResponse {
    pub fn new() -> SpotOrderResponse {
        SpotOrderResponse {
            account_id: None,
            symbol: None,
            client_order_id: None,
            order_id: None,
            transact_time: None,
            price: None,
            orig_qty: None,
            executed_qty: None,
            status: None,
            time_in_force: None,
            r#type: None,
            side: None,
            req_amount: None,
            concentration: None,
        }
    }
}

