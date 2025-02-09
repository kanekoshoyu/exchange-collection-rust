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
pub struct EscrowRequest {
    #[serde(rename = "base_asset", skip_serializing_if = "Option::is_none")]
    pub base_asset: Option<String>,
    #[serde(rename = "base_asset_amount", skip_serializing_if = "Option::is_none")]
    pub base_asset_amount: Option<f32>,
    #[serde(rename = "quote_asset", skip_serializing_if = "Option::is_none")]
    pub quote_asset: Option<String>,
    #[serde(rename = "quote_asset_amount", skip_serializing_if = "Option::is_none")]
    pub quote_asset_amount: Option<f32>,
    #[serde(rename = "counterparty", skip_serializing_if = "Option::is_none")]
    pub counterparty: Option<String>,
}

impl EscrowRequest {
    pub fn new() -> EscrowRequest {
        EscrowRequest {
            base_asset: None,
            base_asset_amount: None,
            quote_asset: None,
            quote_asset_amount: None,
            counterparty: None,
        }
    }
}

