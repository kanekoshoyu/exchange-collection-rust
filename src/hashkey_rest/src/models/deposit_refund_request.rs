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
pub struct DepositRefundRequest {
    /// Deposit Order ID.
    #[serde(rename = "depositOrderId", skip_serializing_if = "Option::is_none")]
    pub deposit_order_id: Option<String>,
    /// Wallet ID.
    #[serde(rename = "walletId", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
}

impl DepositRefundRequest {
    pub fn new() -> DepositRefundRequest {
        DepositRefundRequest {
            deposit_order_id: None,
            wallet_id: None,
        }
    }
}

