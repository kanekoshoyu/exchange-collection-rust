/*
 * Coincheck Crypto Exchange
 *
 * Welcome to Coincheck API document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    /// Indicates success of the API call.
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    /// Account ID. It's the same as the ID you specify when you deposit JPY.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Registered email.
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Your identity status (e.g., identity_pending, verified).
    #[serde(rename = "identity_status", skip_serializing_if = "Option::is_none")]
    pub identity_status: Option<String>,
    /// Your bitcoin address for deposit.
    #[serde(rename = "bitcoin_address", skip_serializing_if = "Option::is_none")]
    pub bitcoin_address: Option<String>,
    /// Fee rate (%) for performing the order as a Taker (BTC_JPY).
    #[serde(rename = "taker_fee", skip_serializing_if = "Option::is_none")]
    pub taker_fee: Option<String>,
    /// Fee rate (%) for performing the order as a Maker (BTC_JPY).
    #[serde(rename = "maker_fee", skip_serializing_if = "Option::is_none")]
    pub maker_fee: Option<String>,
    /// Fees for each order book.
    #[serde(rename = "exchange_fees", skip_serializing_if = "Option::is_none")]
    pub exchange_fees: Option<std::collections::HashMap<String, models::ExchangeFeeValue>>,
}

impl Account {
    pub fn new() -> Account {
        Account {
            success: None,
            id: None,
            email: None,
            identity_status: None,
            bitcoin_address: None,
            taker_fee: None,
            maker_fee: None,
            exchange_fees: None,
        }
    }
}

