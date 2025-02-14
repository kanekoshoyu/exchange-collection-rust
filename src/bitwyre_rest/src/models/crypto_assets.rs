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
pub struct CryptoAssets {
    /// The asset code (e.g., 'btc' for Bitcoin).
    #[serde(rename = "asset", skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    /// The equivalent value in Bitcoin.
    #[serde(rename = "btc_equivalent", skip_serializing_if = "Option::is_none")]
    pub btc_equivalent: Option<String>,
    /// URL to the asset's icon image.
    #[serde(rename = "icon_url", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Indicates if deposit is enabled for the asset.
    #[serde(rename = "is_deposit_enabled", skip_serializing_if = "Option::is_none")]
    pub is_deposit_enabled: Option<bool>,
    /// Indicates if trading is enabled for the asset.
    #[serde(rename = "is_trading_enabled", skip_serializing_if = "Option::is_none")]
    pub is_trading_enabled: Option<bool>,
    /// Indicates if withdrawal is enabled for the asset.
    #[serde(rename = "is_withdraw_enabled", skip_serializing_if = "Option::is_none")]
    pub is_withdraw_enabled: Option<bool>,
    /// The equivalent value in the local currency, if applicable.
    #[serde(rename = "local_equivalent", skip_serializing_if = "Option::is_none")]
    pub local_equivalent: Option<String>,
    /// Local currency reference for the asset.
    #[serde(rename = "local_reference", skip_serializing_if = "Option::is_none")]
    pub local_reference: Option<String>,
    /// The market cap rank of the asset.
    #[serde(rename = "market_cap_rank", skip_serializing_if = "Option::is_none")]
    pub market_cap_rank: Option<String>,
    /// The market capitalization of the asset in USD.
    #[serde(rename = "market_cap_usd", skip_serializing_if = "Option::is_none")]
    pub market_cap_usd: Option<String>,
    /// The maximum amount that can be withdrawn.
    #[serde(rename = "max_withdrawal", skip_serializing_if = "Option::is_none")]
    pub max_withdrawal: Option<String>,
    /// The minimum deposit amount.
    #[serde(rename = "min_deposit", skip_serializing_if = "Option::is_none")]
    pub min_deposit: Option<String>,
    /// The minimum order size for the asset.
    #[serde(rename = "min_order_size", skip_serializing_if = "Option::is_none")]
    pub min_order_size: Option<String>,
    /// The minimum order size increment for the asset.
    #[serde(rename = "min_order_size_increment", skip_serializing_if = "Option::is_none")]
    pub min_order_size_increment: Option<String>,
    /// The minimum price increment in USD.
    #[serde(rename = "min_price_increment_usd", skip_serializing_if = "Option::is_none")]
    pub min_price_increment_usd: Option<String>,
    /// The minimum amount that can be withdrawn.
    #[serde(rename = "min_withdrawal", skip_serializing_if = "Option::is_none")]
    pub min_withdrawal: Option<String>,
    /// The full name of the asset.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "network_list", skip_serializing_if = "Option::is_none")]
    pub network_list: Option<Vec<models::NetworkInfo>>,
    /// The precision for the asset's value, typically in scientific notation.
    #[serde(rename = "precision", skip_serializing_if = "Option::is_none")]
    pub precision: Option<String>,
    /// The price of the asset in USD.
    #[serde(rename = "price_to_usd", skip_serializing_if = "Option::is_none")]
    pub price_to_usd: Option<String>,
    /// The fee applied for withdrawals of the asset.
    #[serde(rename = "withdrawal_fee", skip_serializing_if = "Option::is_none")]
    pub withdrawal_fee: Option<String>,
}

impl CryptoAssets {
    pub fn new() -> CryptoAssets {
        CryptoAssets {
            asset: None,
            btc_equivalent: None,
            icon_url: None,
            is_deposit_enabled: None,
            is_trading_enabled: None,
            is_withdraw_enabled: None,
            local_equivalent: None,
            local_reference: None,
            market_cap_rank: None,
            market_cap_usd: None,
            max_withdrawal: None,
            min_deposit: None,
            min_order_size: None,
            min_order_size_increment: None,
            min_price_increment_usd: None,
            min_withdrawal: None,
            name: None,
            network_list: None,
            precision: None,
            price_to_usd: None,
            withdrawal_fee: None,
        }
    }
}

