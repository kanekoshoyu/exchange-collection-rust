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
pub struct SpotAccountResponse {
    /// The asset to be transferred.
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// The amount of the asset to be transferred.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// Whether to transfer the asset to the derivatives account.
    #[serde(rename = "to_derivatives", skip_serializing_if = "Option::is_none")]
    pub to_derivatives: Option<bool>,
    /// Whether to transfer the asset to the spot account.
    #[serde(rename = "to_spot", skip_serializing_if = "Option::is_none")]
    pub to_spot: Option<bool>,
}

impl SpotAccountResponse {
    pub fn new() -> SpotAccountResponse {
        SpotAccountResponse {
            asset: None,
            amount: None,
            to_derivatives: None,
            to_spot: None,
        }
    }
}

