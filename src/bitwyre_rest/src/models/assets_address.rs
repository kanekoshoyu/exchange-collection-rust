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
pub struct AssetsAddress {
    #[serde(rename = "deposit", skip_serializing_if = "Option::is_none")]
    pub deposit: Option<Box<models::AssetsAddressDeposit>>,
}

impl AssetsAddress {
    pub fn new() -> AssetsAddress {
        AssetsAddress {
            deposit: None,
        }
    }
}

