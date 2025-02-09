#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MultiCollateralResponse represents a MultiCollateralResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct MultiCollateralResponse {
    #[serde(rename="feed")]
    pub feed: String,
    #[serde(rename="account")]
    pub account: String,
    #[serde(rename="flex_futures")]
    pub flex_futures: Box<FlexFuturesSchema>,
    #[serde(rename="timestamp")]
    pub timestamp: i64,
    #[serde(rename="seq")]
    pub seq: i64,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

