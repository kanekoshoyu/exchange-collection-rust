#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FillsPushDataResponse represents a FillsPushDataResponse model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FillsPushDataResponse {
    #[serde(rename="event", skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(rename="arg", skip_serializing_if = "Option::is_none")]
    pub arg: Option<Box<FillsPushDataArg>>,
    #[serde(rename="data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<FillsPushData>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

