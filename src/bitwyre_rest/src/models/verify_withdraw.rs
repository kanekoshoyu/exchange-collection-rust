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
pub struct VerifyWithdraw {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "transaction_id", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(rename = "is_verified", skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<i32>,
}

impl VerifyWithdraw {
    pub fn new() -> VerifyWithdraw {
        VerifyWithdraw {
            id: None,
            transaction_id: None,
            is_verified: None,
        }
    }
}

