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
pub struct TransactionData {
    #[serde(rename = "account_balance_id", skip_serializing_if = "Option::is_none")]
    pub account_balance_id: Option<i32>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f32>,
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<f32>,
    #[serde(rename = "final_balance", skip_serializing_if = "Option::is_none")]
    pub final_balance: Option<f32>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
}

impl TransactionData {
    pub fn new() -> TransactionData {
        TransactionData {
            account_balance_id: None,
            address: None,
            amount: None,
            asset: None,
            fee: None,
            final_balance: None,
            notes: None,
            status: None,
            time: None,
            r#type: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Success")]
    Success,
    #[serde(rename = "Waiting")]
    Waiting,
}

impl Default for Status {
    fn default() -> Status {
        Self::Cancelled
    }
}

