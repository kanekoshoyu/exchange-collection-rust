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
pub struct WithdrawFiatRequest {
    /// The bank account number of the designated bank, must be from the added bank list in your account.
    #[serde(rename = "bankAccount")]
    pub bank_account: String,
    /// Currency type, e.g., 'USD', 'HKD'.
    #[serde(rename = "asset")]
    pub asset: String,
    /// Withdrawal amount.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Remark field (length limit: 128 characters).
    #[serde(rename = "remark", skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// An ID defined by the client for the withdrawal order.
    #[serde(rename = "clientOrderId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// Timestamp in milliseconds.
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
}

impl WithdrawFiatRequest {
    pub fn new(bank_account: String, asset: String, amount: String, timestamp: i64) -> WithdrawFiatRequest {
        WithdrawFiatRequest {
            bank_account,
            asset,
            amount,
            remark: None,
            client_order_id: None,
            timestamp,
        }
    }
}

