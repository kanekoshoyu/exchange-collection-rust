#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// InstrumentArgs represents a InstrumentArgs model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct InstrumentArgs {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<Box<InstTypeEnum>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

