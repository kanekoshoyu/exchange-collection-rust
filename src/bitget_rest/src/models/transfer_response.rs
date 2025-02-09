/*
 * Bitget API
 *
 * Welcome to Bitget Developer document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransferResponse {
    #[serde(rename = "transferId", skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
}

impl TransferResponse {
    pub fn new() -> TransferResponse {
        TransferResponse {
            transfer_id: None,
            client_oid: None,
        }
    }
}

