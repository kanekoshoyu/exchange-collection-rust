#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SpreadResponseData represents a SpreadResponseData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SpreadResponseData {
    #[serde(rename="ordId", skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    #[serde(rename="clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(rename="tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename="sCode", skip_serializing_if = "Option::is_none")]
    pub s_code: Option<String>,
    #[serde(rename="sMsg", skip_serializing_if = "Option::is_none")]
    pub s_msg: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

