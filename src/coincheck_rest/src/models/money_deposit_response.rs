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
pub struct MoneyDepositResponse {
    /// Received ID.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Amount of BTC received.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Currency (e.g., BTC).
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Bitcoin address where BTC was received.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Status of the deposit (e.g., confirmed, received).
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Date and time the deposit was confirmed.
    #[serde(rename = "confirmed_at", skip_serializing_if = "Option::is_none")]
    pub confirmed_at: Option<String>,
    /// Date and time the receiving process started.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl MoneyDepositResponse {
    pub fn new() -> MoneyDepositResponse {
        MoneyDepositResponse {
            id: None,
            amount: None,
            currency: None,
            address: None,
            status: None,
            confirmed_at: None,
            created_at: None,
        }
    }
}

