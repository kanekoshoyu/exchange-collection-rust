/*
 * OKX API
 *
 * Welcome to OKX Developer document!   excluded below endpoints as they are if you need them please add and commit to https://github.com/kanekoshoyu/exchange-collection): - Trading Account (this might become needed, will add when we need it) - Block Trading - Financial Producer - Affiliate - Status - Announcement 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ExercisesHistoryDetails : Delivery/exercise details.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExercisesHistoryDetails {
    /// Delivery/exercise contract ID.
    #[serde(rename = "insId", skip_serializing_if = "Option::is_none")]
    pub ins_id: Option<String>,
    /// Delivery/exercise price.
    #[serde(rename = "px", skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    /// Type of the delivery/exercise. Valid values: - delivery - exercised - expired_otm: Out of the money. 
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ExercisesHistoryDetails {
    /// Delivery/exercise details.
    pub fn new() -> ExercisesHistoryDetails {
        ExercisesHistoryDetails {
            ins_id: None,
            px: None,
            r#type: None,
        }
    }
}

