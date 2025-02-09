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
pub struct InterestResponse {
    /// The coin name (e.g., USDT)
    #[serde(rename = "coin")]
    pub coin: String,
    /// A list of historical interest rates for the specified coin
    #[serde(rename = "historyInterestRateList")]
    pub history_interest_rate_list: Vec<models::InterestResponseHistoryInterestRateListInner>,
}

impl InterestResponse {
    pub fn new(coin: String, history_interest_rate_list: Vec<models::InterestResponseHistoryInterestRateListInner>) -> InterestResponse {
        InterestResponse {
            coin,
            history_interest_rate_list,
        }
    }
}

