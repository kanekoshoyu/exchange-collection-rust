#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AmendOrderArgs represents a AmendOrderArgs model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AmendOrderArgs {
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="cxlOnFail", skip_serializing_if = "Option::is_none")]
    pub cxl_on_fail: Option<bool>,
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
    #[serde(rename="newPxUsd", skip_serializing_if = "Option::is_none")]
    pub new_px_usd: Option<String>,
    #[serde(rename="newPxVol", skip_serializing_if = "Option::is_none")]
    pub new_px_vol: Option<String>,
    #[serde(rename="expTime", skip_serializing_if = "Option::is_none")]
    pub exp_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

