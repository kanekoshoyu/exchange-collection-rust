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
pub struct SpotAccountRequest {
    /// the asset to be moved from the spot balance to the derivatives account balance
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// the amount of the asset to be moved from the spot balance to the derivatives account balance (in the currency's normal unit e.g bitcoin, ether)
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
}

impl SpotAccountRequest {
    pub fn new() -> SpotAccountRequest {
        SpotAccountRequest {
            asset: None,
            amount: None,
        }
    }
}

