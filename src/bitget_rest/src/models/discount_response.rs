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
pub struct DiscountResponse {
    /// The coin name (e.g., BTC)
    #[serde(rename = "coin")]
    pub coin: String,
    /// The user's limit for the coin
    #[serde(rename = "userLimit")]
    pub user_limit: String,
    /// The total limit for the coin
    #[serde(rename = "totalLimit")]
    pub total_limit: String,
    /// A list of discount rates based on tiers and amounts
    #[serde(rename = "discountRateList")]
    pub discount_rate_list: Vec<models::DiscountResponseDiscountRateListInner>,
}

impl DiscountResponse {
    pub fn new(coin: String, user_limit: String, total_limit: String, discount_rate_list: Vec<models::DiscountResponseDiscountRateListInner>) -> DiscountResponse {
        DiscountResponse {
            coin,
            user_limit,
            total_limit,
            discount_rate_list,
        }
    }
}

