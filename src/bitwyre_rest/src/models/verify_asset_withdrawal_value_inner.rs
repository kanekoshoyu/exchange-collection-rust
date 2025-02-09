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
pub struct VerifyAssetWithdrawalValueInner {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<f32>,
    #[serde(rename = "gross_amount", skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<f32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "is_verified", skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<i32>,
    #[serde(rename = "nett_amount", skip_serializing_if = "Option::is_none")]
    pub nett_amount: Option<f32>,
    #[serde(rename = "network_confirmation", skip_serializing_if = "Option::is_none")]
    pub network_confirmation: Option<i32>,
    #[serde(rename = "network_id", skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[serde(rename = "network_name", skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "submit_time", skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    #[serde(rename = "success_time", skip_serializing_if = "Option::is_none")]
    pub success_time: Option<i32>,
    #[serde(rename = "tx_id", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    #[serde(rename = "tx_hash_url", skip_serializing_if = "Option::is_none")]
    pub tx_hash_url: Option<String>,
    #[serde(rename = "verification_uuid", skip_serializing_if = "Option::is_none")]
    pub verification_uuid: Option<uuid::Uuid>,
    #[serde(rename = "withdrawal_type", skip_serializing_if = "Option::is_none")]
    pub withdrawal_type: Option<String>,
}

impl VerifyAssetWithdrawalValueInner {
    pub fn new() -> VerifyAssetWithdrawalValueInner {
        VerifyAssetWithdrawalValueInner {
            asset: None,
            fee: None,
            gross_amount: None,
            id: None,
            is_verified: None,
            nett_amount: None,
            network_confirmation: None,
            network_id: None,
            network_name: None,
            provider: None,
            status: None,
            submit_time: None,
            success_time: None,
            tx_id: None,
            tx_hash_url: None,
            verification_uuid: None,
            withdrawal_type: None,
        }
    }
}

