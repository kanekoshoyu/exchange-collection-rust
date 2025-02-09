/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapiV1AssetDustBtcPost200Response {
    #[serde(rename = "details")]
    pub details: Vec<models::SapiV1AssetDustBtcPost200ResponseDetailsInner>,
    #[serde(rename = "totalTransferBtc")]
    pub total_transfer_btc: String,
    #[serde(rename = "totalTransferBNB")]
    pub total_transfer_bnb: String,
    /// Commission fee
    #[serde(rename = "dribbletPercentage")]
    pub dribblet_percentage: String,
}

impl SapiV1AssetDustBtcPost200Response {
    pub fn new(details: Vec<models::SapiV1AssetDustBtcPost200ResponseDetailsInner>, total_transfer_btc: String, total_transfer_bnb: String, dribblet_percentage: String) -> SapiV1AssetDustBtcPost200Response {
        SapiV1AssetDustBtcPost200Response {
            details,
            total_transfer_btc,
            total_transfer_bnb,
            dribblet_percentage,
        }
    }
}

