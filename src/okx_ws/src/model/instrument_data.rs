#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// InstrumentData represents a InstrumentData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct InstrumentData {
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="uly", skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(rename="instFamily", skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    #[serde(rename="baseCcy", skip_serializing_if = "Option::is_none")]
    pub base_ccy: Option<String>,
    #[serde(rename="quoteCcy", skip_serializing_if = "Option::is_none")]
    pub quote_ccy: Option<String>,
    #[serde(rename="tickSz", skip_serializing_if = "Option::is_none")]
    pub tick_sz: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

