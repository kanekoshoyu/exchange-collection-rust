#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SpreadAmendArgs represents a SpreadAmendArgs model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SpreadAmendArgs {
    #[serde(rename="ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(rename="clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(rename="reqId", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    #[serde(rename="newSz", skip_serializing_if = "Option::is_none")]
    pub new_sz: Option<String>,
    #[serde(rename="newPx", skip_serializing_if = "Option::is_none")]
    pub new_px: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

