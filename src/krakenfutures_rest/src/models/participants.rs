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
pub struct Participants {
    /// Unique identifier for the participant.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    /// Details about the participant.
    #[serde(rename = "participant", skip_serializing_if = "Option::is_none")]
    pub participant: Option<serde_json::Value>,
    /// Type of contract associated with the participant.
    #[serde(rename = "contractType", skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<String>,
    /// Contract details, nullable if not specified.
    #[serde(rename = "contract", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub contract: Option<Option<String>>,
    /// Maximum allowable size for the participant.
    #[serde(rename = "maxSize", skip_serializing_if = "Option::is_none")]
    pub max_size: Option<f64>,
    /// Maximum position size for the participant.
    #[serde(rename = "maxPosition", skip_serializing_if = "Option::is_none")]
    pub max_position: Option<f64>,
    /// Indicates if the participant accepts long positions.
    #[serde(rename = "acceptLong", skip_serializing_if = "Option::is_none")]
    pub accept_long: Option<bool>,
    /// Indicates if the participant accepts short positions.
    #[serde(rename = "acceptShort", skip_serializing_if = "Option::is_none")]
    pub accept_short: Option<bool>,
    /// Time frame applicable for the participant.
    #[serde(rename = "timeFrame", skip_serializing_if = "Option::is_none")]
    pub time_frame: Option<TimeFrame>,
    /// Indicates if the participant is enabled.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl Participants {
    pub fn new() -> Participants {
        Participants {
            id: None,
            participant: None,
            contract_type: None,
            contract: None,
            max_size: None,
            max_position: None,
            accept_long: None,
            accept_short: None,
            time_frame: None,
            enabled: None,
        }
    }
}
/// Time frame applicable for the participant.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TimeFrame {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "weekdays")]
    Weekdays,
    #[serde(rename = "weekends")]
    Weekends,
}

impl Default for TimeFrame {
    fn default() -> TimeFrame {
        Self::All
    }
}

