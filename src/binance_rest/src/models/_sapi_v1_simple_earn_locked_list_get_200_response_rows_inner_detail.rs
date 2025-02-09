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
pub struct SapiV1SimpleEarnLockedListGet200ResponseRowsInnerDetail {
    #[serde(rename = "asset")]
    pub asset: String,
    #[serde(rename = "rewardAsset")]
    pub reward_asset: String,
    #[serde(rename = "duration")]
    pub duration: i64,
    #[serde(rename = "renewable")]
    pub renewable: bool,
    #[serde(rename = "isSoldOut")]
    pub is_sold_out: bool,
    #[serde(rename = "apr")]
    pub apr: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "subscriptionStartTime")]
    pub subscription_start_time: String,
    #[serde(rename = "extraRewardAsset")]
    pub extra_reward_asset: String,
    #[serde(rename = "extraRewardAPR")]
    pub extra_reward_apr: String,
}

impl SapiV1SimpleEarnLockedListGet200ResponseRowsInnerDetail {
    pub fn new(asset: String, reward_asset: String, duration: i64, renewable: bool, is_sold_out: bool, apr: String, status: String, subscription_start_time: String, extra_reward_asset: String, extra_reward_apr: String) -> SapiV1SimpleEarnLockedListGet200ResponseRowsInnerDetail {
        SapiV1SimpleEarnLockedListGet200ResponseRowsInnerDetail {
            asset,
            reward_asset,
            duration,
            renewable,
            is_sold_out,
            apr,
            status,
            subscription_start_time,
            extra_reward_asset,
            extra_reward_apr,
        }
    }
}

