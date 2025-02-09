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
pub struct SapiV1ManagedSubaccountFetchFutureAssetGet200ResponseSnapshotVosInnerDataAssetsInner {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "marginBalance")]
    pub margin_balance: f32,
    #[serde(rename = "walletBalance")]
    pub wallet_balance: f32,
}

impl SapiV1ManagedSubaccountFetchFutureAssetGet200ResponseSnapshotVosInnerDataAssetsInner {
    pub fn new(asset: String, margin_balance: f32, wallet_balance: f32) -> SapiV1ManagedSubaccountFetchFutureAssetGet200ResponseSnapshotVosInnerDataAssetsInner {
        SapiV1ManagedSubaccountFetchFutureAssetGet200ResponseSnapshotVosInnerDataAssetsInner {
            asset,
            margin_balance,
            wallet_balance,
        }
    }
}

