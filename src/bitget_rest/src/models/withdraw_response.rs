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
pub struct WithdrawResponse {
    /// Unique order ID
    #[serde(rename = "orderId")]
    pub order_id: String,
    /// Unique trade ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    /// Coin name (e.g., USDT)
    #[serde(rename = "coin")]
    pub coin: String,
    /// Destination of the transfer (e.g., external, internal)
    #[serde(rename = "dest")]
    pub dest: String,
    /// Client customized ID
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
    /// The type of transaction (e.g., withdraw)
    #[serde(rename = "type")]
    pub r#type: String,
    /// Tag for certain coins, such as EOS
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Transaction amount
    #[serde(rename = "size")]
    pub size: String,
    /// Transaction fee (can be negative in case of refund)
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
    /// Transaction status (e.g., success, failed)
    #[serde(rename = "status")]
    pub status: String,
    /// Recipient address
    #[serde(rename = "toAddress")]
    pub to_address: String,
    /// Sender address
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    /// Number of confirmations required for the transaction
    #[serde(rename = "confirm")]
    pub confirm: String,
    /// Blockchain network (e.g., ERC20, TRC20)
    #[serde(rename = "chain")]
    pub chain: String,
    /// Creation timestamp (Unix millisecond timestamp)
    #[serde(rename = "cTime")]
    pub c_time: String,
    /// Last updated timestamp (Unix millisecond timestamp)
    #[serde(rename = "uTime")]
    pub u_time: String,
}

impl WithdrawResponse {
    pub fn new(order_id: String, trade_id: String, coin: String, dest: String, r#type: String, size: String, status: String, to_address: String, from_address: String, confirm: String, chain: String, c_time: String, u_time: String) -> WithdrawResponse {
        WithdrawResponse {
            order_id,
            trade_id,
            coin,
            dest,
            client_oid: None,
            r#type,
            tag: None,
            size,
            fee: None,
            status,
            to_address,
            from_address,
            confirm,
            chain,
            c_time,
            u_time,
        }
    }
}

