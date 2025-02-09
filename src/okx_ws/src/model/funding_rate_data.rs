#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FundingRateData represents a FundingRateData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FundingRateData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="method", skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename="fundingRate", skip_serializing_if = "Option::is_none")]
    pub funding_rate: Option<String>,
    #[serde(rename="fundingTime", skip_serializing_if = "Option::is_none")]
    pub funding_time: Option<String>,
    #[serde(rename="nextFundingRate", skip_serializing_if = "Option::is_none")]
    pub next_funding_rate: Option<String>,
    #[serde(rename="nextFundingTime", skip_serializing_if = "Option::is_none")]
    pub next_funding_time: Option<String>,
    #[serde(rename="minFundingRate", skip_serializing_if = "Option::is_none")]
    pub min_funding_rate: Option<String>,
    #[serde(rename="maxFundingRate", skip_serializing_if = "Option::is_none")]
    pub max_funding_rate: Option<String>,
    #[serde(rename="settState", skip_serializing_if = "Option::is_none")]
    pub sett_state: Option<String>,
    #[serde(rename="settFundingRate", skip_serializing_if = "Option::is_none")]
    pub sett_funding_rate: Option<String>,
    #[serde(rename="premium", skip_serializing_if = "Option::is_none")]
    pub premium: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

