#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SpreadCancelArgs represents a SpreadCancelArgs model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SpreadCancelArgs {
    #[serde(rename="ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(rename="clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

