#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Args represents a Args model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Args {
    #[serde(rename="channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="extraParams", skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<Box<ExtraParams>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

