/*
 * Coinbase API
 *
 * The Coinbase v2 API (converted from Swagger to OpenAPI by kanekoshoyu)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    /// Resource ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Bitcoin address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Timestamp indicating when was the address was created.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Timestamp indicating when was the address last updated.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// This value is always 'address'.
    #[serde(rename = "resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resource_path", skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
}

impl Address {
    pub fn new() -> Address {
        Address {
            id: None,
            address: None,
            created_at: None,
            updated_at: None,
            resource: None,
            resource_path: None,
        }
    }
}

