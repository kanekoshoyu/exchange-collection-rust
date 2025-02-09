/*
 * Kraken API
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
pub struct FeeScheduleItems {
    /// The name of the fee schedule.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Unique identifier of the fee schedule.
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// A list of fee tiers.
    #[serde(rename = "tiers", skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<models::FeeTier>>,
}

impl FeeScheduleItems {
    pub fn new() -> FeeScheduleItems {
        FeeScheduleItems {
            name: None,
            uid: None,
            tiers: None,
        }
    }
}

