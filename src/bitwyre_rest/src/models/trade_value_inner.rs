/*
 * Bitwyre REST API
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
pub struct TradeValueInner {
    /// The price at which the execution occurred.
    #[serde(rename = "exec_price", skip_serializing_if = "Option::is_none")]
    pub exec_price: Option<f64>,
    /// The quantity executed in the trade.
    #[serde(rename = "exec_qty", skip_serializing_if = "Option::is_none")]
    pub exec_qty: Option<f64>,
    /// The type of execution.
    #[serde(rename = "exec_type", skip_serializing_if = "Option::is_none")]
    pub exec_type: Option<i32>,
    /// The total value of the executed trade.
    #[serde(rename = "exec_value", skip_serializing_if = "Option::is_none")]
    pub exec_value: Option<f64>,
    /// The fee paid for the execution.
    #[serde(rename = "fee_paid", skip_serializing_if = "Option::is_none")]
    pub fee_paid: Option<f64>,
    /// The asset in which the fee was paid.
    #[serde(rename = "fee_paid_asset", skip_serializing_if = "Option::is_none")]
    pub fee_paid_asset: Option<String>,
    /// The rate of the fee applied.
    #[serde(rename = "fee_rate", skip_serializing_if = "Option::is_none")]
    pub fee_rate: Option<f64>,
    /// The type of fee applied.
    #[serde(rename = "fee_type", skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<i32>,
    /// Any additional notes for the execution.
    #[serde(rename = "notes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub notes: Option<Option<String>>,
    /// The price of the order.
    #[serde(rename = "order_price", skip_serializing_if = "Option::is_none")]
    pub order_price: Option<f64>,
    /// The quantity ordered.
    #[serde(rename = "order_qty", skip_serializing_if = "Option::is_none")]
    pub order_qty: Option<f64>,
    /// The remaining quantity of the order.
    #[serde(rename = "order_remaining", skip_serializing_if = "Option::is_none")]
    pub order_remaining: Option<f64>,
    /// The type of order (e.g., limit, market).
    #[serde(rename = "order_type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<i32>,
    /// The side of the trade, where 1 represents buy and 0 represents sell.
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<i32>,
    /// The status of the order execution.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// The timestamp of the execution in Unix nanoseconds.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
    /// The username associated with the execution.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl TradeValueInner {
    pub fn new() -> TradeValueInner {
        TradeValueInner {
            exec_price: None,
            exec_qty: None,
            exec_type: None,
            exec_value: None,
            fee_paid: None,
            fee_paid_asset: None,
            fee_rate: None,
            fee_type: None,
            notes: None,
            order_price: None,
            order_qty: None,
            order_remaining: None,
            order_type: None,
            side: None,
            status: None,
            timestamp: None,
            username: None,
        }
    }
}

