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
pub struct PaymentMethods {
    #[serde(rename = "fiat", skip_serializing_if = "Option::is_none")]
    pub fiat: Option<Vec<String>>,
    #[serde(rename = "crypto", skip_serializing_if = "Option::is_none")]
    pub crypto: Option<Vec<String>>,
}

impl PaymentMethods {
    pub fn new() -> PaymentMethods {
        PaymentMethods {
            fiat: None,
            crypto: None,
        }
    }
}

