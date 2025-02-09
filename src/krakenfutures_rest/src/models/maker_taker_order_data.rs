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
pub struct MakerTakerOrderData {
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<f64>,
    #[serde(rename = "positionSize", skip_serializing_if = "Option::is_none")]
    pub position_size: Option<f64>,
}

impl MakerTakerOrderData {
    pub fn new() -> MakerTakerOrderData {
        MakerTakerOrderData {
            fee: None,
            position_size: None,
        }
    }
}

