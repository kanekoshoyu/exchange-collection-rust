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
pub struct SapiV1ConvertExchangeInfoGet200ResponseInner {
    #[serde(rename = "fromAsset")]
    pub from_asset: String,
    #[serde(rename = "toAsset")]
    pub to_asset: String,
    #[serde(rename = "fromAssetMinAmount")]
    pub from_asset_min_amount: String,
    #[serde(rename = "fromAssetMaxAmount")]
    pub from_asset_max_amount: String,
    #[serde(rename = "toAssetMinAmount")]
    pub to_asset_min_amount: String,
    #[serde(rename = "toAssetMaxAmount")]
    pub to_asset_max_amount: String,
}

impl SapiV1ConvertExchangeInfoGet200ResponseInner {
    pub fn new(from_asset: String, to_asset: String, from_asset_min_amount: String, from_asset_max_amount: String, to_asset_min_amount: String, to_asset_max_amount: String) -> SapiV1ConvertExchangeInfoGet200ResponseInner {
        SapiV1ConvertExchangeInfoGet200ResponseInner {
            from_asset,
            to_asset,
            from_asset_min_amount,
            from_asset_max_amount,
            to_asset_min_amount,
            to_asset_max_amount,
        }
    }
}

