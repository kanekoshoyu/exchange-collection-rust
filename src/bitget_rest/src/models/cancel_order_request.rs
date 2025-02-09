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

/// CancelOrderRequest : Query parameters for retrieving or modifying a specific order based on trading pair, product type, and optional identifiers.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    /// Trading pair, e.g., \"ETHUSDT\".
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Type of futures product.
    #[serde(rename = "productType")]
    pub product_type: ProductType,
    /// Margin coin, which must be capitalized.
    #[serde(rename = "marginCoin", skip_serializing_if = "Option::is_none")]
    pub margin_coin: Option<String>,
    /// Order ID. Either `orderId` or `clientOid` is required. If both are present, `orderId` prevails.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Custom order ID. Either `orderId` or `clientOid` is required. If both are present, `orderId` prevails.
    #[serde(rename = "clientOid", skip_serializing_if = "Option::is_none")]
    pub client_oid: Option<String>,
}

impl CancelOrderRequest {
    /// Query parameters for retrieving or modifying a specific order based on trading pair, product type, and optional identifiers.
    pub fn new(symbol: String, product_type: ProductType) -> CancelOrderRequest {
        CancelOrderRequest {
            symbol,
            product_type,
            margin_coin: None,
            order_id: None,
            client_oid: None,
        }
    }
}
/// Type of futures product.
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

