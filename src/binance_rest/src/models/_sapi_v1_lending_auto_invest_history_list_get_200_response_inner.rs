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
pub struct SapiV1LendingAutoInvestHistoryListGet200ResponseInner {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "targetAsset")]
    pub target_asset: String,
    #[serde(rename = "planType")]
    pub plan_type: String,
    #[serde(rename = "planName")]
    pub plan_name: String,
    #[serde(rename = "planId")]
    pub plan_id: i64,
    #[serde(rename = "transactionDateTime")]
    pub transaction_date_time: i64,
    #[serde(rename = "transactionStatus")]
    pub transaction_status: String,
    #[serde(rename = "failedType")]
    pub failed_type: String,
    #[serde(rename = "sourceAsset")]
    pub source_asset: String,
    #[serde(rename = "sourceAssetAmount")]
    pub source_asset_amount: String,
    #[serde(rename = "targetAssetAmount")]
    pub target_asset_amount: String,
    #[serde(rename = "sourceWallet")]
    pub source_wallet: String,
    #[serde(rename = "flexibleUsed")]
    pub flexible_used: String,
    #[serde(rename = "transactionFee")]
    pub transaction_fee: String,
    #[serde(rename = "transactionFeeUnit")]
    pub transaction_fee_unit: String,
    #[serde(rename = "executionPrice")]
    pub execution_price: String,
    #[serde(rename = "executionType")]
    pub execution_type: String,
    #[serde(rename = "subscriptionCycle")]
    pub subscription_cycle: String,
}

impl SapiV1LendingAutoInvestHistoryListGet200ResponseInner {
    pub fn new(id: i64, target_asset: String, plan_type: String, plan_name: String, plan_id: i64, transaction_date_time: i64, transaction_status: String, failed_type: String, source_asset: String, source_asset_amount: String, target_asset_amount: String, source_wallet: String, flexible_used: String, transaction_fee: String, transaction_fee_unit: String, execution_price: String, execution_type: String, subscription_cycle: String) -> SapiV1LendingAutoInvestHistoryListGet200ResponseInner {
        SapiV1LendingAutoInvestHistoryListGet200ResponseInner {
            id,
            target_asset,
            plan_type,
            plan_name,
            plan_id,
            transaction_date_time,
            transaction_status,
            failed_type,
            source_asset,
            source_asset_amount,
            target_asset_amount,
            source_wallet,
            flexible_used,
            transaction_fee,
            transaction_fee_unit,
            execution_price,
            execution_type,
            subscription_cycle,
        }
    }
}

