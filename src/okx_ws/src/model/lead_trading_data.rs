#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// LeadTradingData represents a LeadTradingData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct LeadTradingData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="infoType", skip_serializing_if = "Option::is_none")]
    pub info_type: Option<String>,
    #[serde(rename="subPosId", skip_serializing_if = "Option::is_none")]
    pub sub_pos_id: Option<String>,
    #[serde(rename="uniqueCode", skip_serializing_if = "Option::is_none")]
    pub unique_code: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
    #[serde(rename="maxLeadTraderNum", skip_serializing_if = "Option::is_none")]
    pub max_lead_trader_num: Option<String>,
    #[serde(rename="minLeadEq", skip_serializing_if = "Option::is_none")]
    pub min_lead_eq: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

