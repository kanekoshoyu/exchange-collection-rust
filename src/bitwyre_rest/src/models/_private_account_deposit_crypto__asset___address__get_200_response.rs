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
pub struct PrivateAccountDepositCryptoAssetAddressGet200Response {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Vec<String>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<models::Meta>>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Box<models::AssetAddress>>,
}

impl PrivateAccountDepositCryptoAssetAddressGet200Response {
    pub fn new() -> PrivateAccountDepositCryptoAssetAddressGet200Response {
        PrivateAccountDepositCryptoAssetAddressGet200Response {
            error: None,
            meta: None,
            result: None,
        }
    }
}

