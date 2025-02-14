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
pub struct SetAssetRequest {
    /// Product type
    #[serde(rename = "productType", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
    #[serde(rename = "assetMode", skip_serializing_if = "Option::is_none")]
    pub asset_mode: Option<AssetMode>,
}

impl SetAssetRequest {
    pub fn new() -> SetAssetRequest {
        SetAssetRequest {
            product_type: None,
            asset_mode: None,
        }
    }
}
/// Product type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProductType {
    #[serde(rename = "USDT-FUTURES")]
    UsdtFutures,
    #[serde(rename = "COIN-FUTURES")]
    CoinFutures,
    #[serde(rename = "USDC-FUTURES")]
    UsdcFutures,
    #[serde(rename = "SUSDT-FUTURES")]
    SusdtFutures,
    #[serde(rename = "SCOIN-FUTURES")]
    ScoinFutures,
    #[serde(rename = "SUSDC-FUTURES")]
    SusdcFutures,
}

impl Default for ProductType {
    fn default() -> ProductType {
        Self::UsdtFutures
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssetMode {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "union")]
    Union,
}

impl Default for AssetMode {
    fn default() -> AssetMode {
        Self::Single
    }
}

