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
pub struct VaultResponse {
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "network_name", skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    #[serde(rename = "deposit_address", skip_serializing_if = "Option::is_none")]
    pub deposit_address: Option<String>,
    #[serde(rename = "address_tag", skip_serializing_if = "Option::is_none")]
    pub address_tag: Option<String>,
    #[serde(rename = "address_tag_name", skip_serializing_if = "Option::is_none")]
    pub address_tag_name: Option<String>,
}

impl VaultResponse {
    pub fn new() -> VaultResponse {
        VaultResponse {
            asset: None,
            network_name: None,
            deposit_address: None,
            address_tag: None,
            address_tag_name: None,
        }
    }
}

