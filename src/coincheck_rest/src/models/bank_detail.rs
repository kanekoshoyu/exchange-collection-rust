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
pub struct BankDetail {
    /// ID of the bank account.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Name of the bank.
    #[serde(rename = "bank_name", skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    /// Name of the branch.
    #[serde(rename = "branch_name", skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// Type of the bank account (e.g., futsu for regular accounts).
    #[serde(rename = "bank_account_type", skip_serializing_if = "Option::is_none")]
    pub bank_account_type: Option<String>,
    /// Bank account number.
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// Name of the bank account holder.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl BankDetail {
    pub fn new() -> BankDetail {
        BankDetail {
            id: None,
            bank_name: None,
            branch_name: None,
            bank_account_type: None,
            number: None,
            name: None,
        }
    }
}

