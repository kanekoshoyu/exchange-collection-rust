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
pub struct SapiV1LoanLoanableDataGet200ResponseRowsInner {
    #[serde(rename = "loanCoin")]
    pub loan_coin: String,
    #[serde(rename = "_7dHourlyInterestRate")]
    pub _7d_hourly_interest_rate: String,
    #[serde(rename = "_7dDailyInterestRate")]
    pub _7d_daily_interest_rate: String,
    #[serde(rename = "_14dHourlyInterestRate")]
    pub _14d_hourly_interest_rate: String,
    #[serde(rename = "_14dDailyInterestRate")]
    pub _14d_daily_interest_rate: String,
    #[serde(rename = "_30dHourlyInterestRate")]
    pub _30d_hourly_interest_rate: String,
    #[serde(rename = "_30dDailyInterestRate")]
    pub _30d_daily_interest_rate: String,
    #[serde(rename = "_90dHourlyInterestRate")]
    pub _90d_hourly_interest_rate: String,
    #[serde(rename = "_90dDailyInterestRate")]
    pub _90d_daily_interest_rate: String,
    #[serde(rename = "_180dHourlyInterestRate")]
    pub _180d_hourly_interest_rate: String,
    #[serde(rename = "_180dDailyInterestRate")]
    pub _180d_daily_interest_rate: String,
    #[serde(rename = "minLimit")]
    pub min_limit: String,
    #[serde(rename = "maxLimit")]
    pub max_limit: String,
    #[serde(rename = "vipLevel")]
    pub vip_level: i32,
}

impl SapiV1LoanLoanableDataGet200ResponseRowsInner {
    pub fn new(loan_coin: String, _7d_hourly_interest_rate: String, _7d_daily_interest_rate: String, _14d_hourly_interest_rate: String, _14d_daily_interest_rate: String, _30d_hourly_interest_rate: String, _30d_daily_interest_rate: String, _90d_hourly_interest_rate: String, _90d_daily_interest_rate: String, _180d_hourly_interest_rate: String, _180d_daily_interest_rate: String, min_limit: String, max_limit: String, vip_level: i32) -> SapiV1LoanLoanableDataGet200ResponseRowsInner {
        SapiV1LoanLoanableDataGet200ResponseRowsInner {
            loan_coin,
            _7d_hourly_interest_rate,
            _7d_daily_interest_rate,
            _14d_hourly_interest_rate,
            _14d_daily_interest_rate,
            _30d_hourly_interest_rate,
            _30d_daily_interest_rate,
            _90d_hourly_interest_rate,
            _90d_daily_interest_rate,
            _180d_hourly_interest_rate,
            _180d_daily_interest_rate,
            min_limit,
            max_limit,
            vip_level,
        }
    }
}

