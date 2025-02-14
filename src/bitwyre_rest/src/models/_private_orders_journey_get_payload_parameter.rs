/*
 * Bitwyre REST API
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
pub struct PrivateOrdersJourneyGetPayloadParameter {
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}

impl PrivateOrdersJourneyGetPayloadParameter {
    pub fn new() -> PrivateOrdersJourneyGetPayloadParameter {
        PrivateOrdersJourneyGetPayloadParameter {
            order_id: None,
        }
    }
}

