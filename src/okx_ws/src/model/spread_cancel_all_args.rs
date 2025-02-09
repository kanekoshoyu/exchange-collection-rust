#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SpreadCancelAllArgs represents a SpreadCancelAllArgs model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SpreadCancelAllArgs {
    #[serde(rename="sprdId", skip_serializing_if = "Option::is_none")]
    pub sprd_id: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

