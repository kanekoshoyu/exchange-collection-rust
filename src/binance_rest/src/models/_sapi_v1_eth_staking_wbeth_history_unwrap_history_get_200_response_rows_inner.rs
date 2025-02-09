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
pub struct SapiV1EthStakingWbethHistoryUnwrapHistoryGet200ResponseRowsInner {
    #[serde(rename = "time")]
    pub time: i64,
    #[serde(rename = "fromAsset")]
    pub from_asset: String,
    #[serde(rename = "fromAmount")]
    pub from_amount: String,
    #[serde(rename = "toAsset")]
    pub to_asset: String,
    #[serde(rename = "toAmount")]
    pub to_amount: String,
    /// BETH amount per 1 WBETH
    #[serde(rename = "exchangeRate")]
    pub exchange_rate: String,
    /// PENDING, SUCCESS, FAILED
    #[serde(rename = "status")]
    pub status: String,
}

impl SapiV1EthStakingWbethHistoryUnwrapHistoryGet200ResponseRowsInner {
    pub fn new(time: i64, from_asset: String, from_amount: String, to_asset: String, to_amount: String, exchange_rate: String, status: String) -> SapiV1EthStakingWbethHistoryUnwrapHistoryGet200ResponseRowsInner {
        SapiV1EthStakingWbethHistoryUnwrapHistoryGet200ResponseRowsInner {
            time,
            from_asset,
            from_amount,
            to_asset,
            to_amount,
            exchange_rate,
            status,
        }
    }
}

