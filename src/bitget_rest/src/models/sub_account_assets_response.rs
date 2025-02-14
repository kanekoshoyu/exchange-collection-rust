/*
 * Bitget API
 *
 * Welcome to Bitget Developer document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubAccountAssetsResponse {
    /// User ID
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,
    #[serde(rename = "assetsList", skip_serializing_if = "Option::is_none")]
    pub assets_list: Option<Vec<models::AssetDetail>>,
}

impl SubAccountAssetsResponse {
    pub fn new() -> SubAccountAssetsResponse {
        SubAccountAssetsResponse {
            user_id: None,
            assets_list: None,
        }
    }
}

