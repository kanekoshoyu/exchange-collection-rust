/*
 * MEXC Crypto Exchange
 *
 * Welcome to mexc API document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserUniversalTransfer200Response {
    /// The transaction ID of the transfer.
    #[serde(rename = "tranId", skip_serializing_if = "Option::is_none")]
    pub tran_id: Option<String>,
}

impl UserUniversalTransfer200Response {
    pub fn new() -> UserUniversalTransfer200Response {
        UserUniversalTransfer200Response {
            tran_id: None,
        }
    }
}

