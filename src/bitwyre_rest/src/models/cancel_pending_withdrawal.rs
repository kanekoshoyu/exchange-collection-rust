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
pub struct CancelPendingWithdrawal {
    #[serde(rename = "withdrawal", skip_serializing_if = "Option::is_none")]
    pub withdrawal: Option<std::collections::HashMap<String, Vec<models::CancelPendingWithdrawalWithdrawalValueInner>>>,
}

impl CancelPendingWithdrawal {
    pub fn new() -> CancelPendingWithdrawal {
        CancelPendingWithdrawal {
            withdrawal: None,
        }
    }
}

