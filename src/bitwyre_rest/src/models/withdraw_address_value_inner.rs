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
pub struct WithdrawAddressValueInner {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "address_name", skip_serializing_if = "Option::is_none")]
    pub address_name: Option<String>,
    #[serde(rename = "address_tag", skip_serializing_if = "Option::is_none")]
    pub address_tag: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl WithdrawAddressValueInner {
    pub fn new() -> WithdrawAddressValueInner {
        WithdrawAddressValueInner {
            address: None,
            address_name: None,
            address_tag: None,
            created_at: None,
        }
    }
}

