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
pub struct LeverageRequest {
    /// Trading pair
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Product type
    #[serde(rename = "productType", skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
    /// Margin coin (must be capitalized)
    #[serde(rename = "marginCoin", skip_serializing_if = "Option::is_none")]
    pub margin_coin: Option<String>,
    /// Leverage
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    /// Position direction (ignored in crossed margin mode)
    #[serde(rename = "holdSide", skip_serializing_if = "Option::is_none")]
    pub hold_side: Option<HoldSide>,
}

impl LeverageRequest {
    pub fn new() -> LeverageRequest {
        LeverageRequest {
            symbol: None,
            product_type: None,
            margin_coin: None,
            leverage: None,
            hold_side: None,
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
/// Position direction (ignored in crossed margin mode)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HoldSide {
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "short")]
    Short,
}

impl Default for HoldSide {
    fn default() -> HoldSide {
        Self::Long
    }
}

