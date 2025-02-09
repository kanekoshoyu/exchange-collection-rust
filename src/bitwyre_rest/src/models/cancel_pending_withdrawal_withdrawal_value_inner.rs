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
pub struct CancelPendingWithdrawalWithdrawalValueInner {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "submit_time", skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    #[serde(rename = "success_time", skip_serializing_if = "Option::is_none")]
    pub success_time: Option<String>,
}

impl CancelPendingWithdrawalWithdrawalValueInner {
    pub fn new() -> CancelPendingWithdrawalWithdrawalValueInner {
        CancelPendingWithdrawalWithdrawalValueInner {
            id: None,
            status: None,
            submit_time: None,
            success_time: None,
        }
    }
}

