#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PlaceOrderArgs represents a PlaceOrderArgs model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct PlaceOrderArgs {
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="tdMode", skip_serializing_if = "Option::is_none")]
    pub td_mode: Option<String>,
    #[serde(rename="ccy", skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    #[serde(rename="clOrdId", skip_serializing_if = "Option::is_none")]
    pub cl_ord_id: Option<String>,
    #[serde(rename="tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename="side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename="posSide", skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
    #[serde(rename="ordType", skip_serializing_if = "Option::is_none")]
    pub ord_type: Option<Box<OrderEnum>>,
    #[serde(rename="sz", skip_serializing_if = "Option::is_none")]
    pub sz: Option<String>,
    #[serde(rename="px", skip_serializing_if = "Option::is_none")]
    pub px: Option<String>,
    #[serde(rename="pxUsd", skip_serializing_if = "Option::is_none")]
    pub px_usd: Option<String>,
    #[serde(rename="pxVol", skip_serializing_if = "Option::is_none")]
    pub px_vol: Option<String>,
    #[serde(rename="reduceOnly", skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename="tgtCcy", skip_serializing_if = "Option::is_none")]
    pub tgt_ccy: Option<String>,
    #[serde(rename="banAmend", skip_serializing_if = "Option::is_none")]
    pub ban_amend: Option<bool>,
    #[serde(rename="quickMgnType", skip_serializing_if = "Option::is_none")]
    pub quick_mgn_type: Option<String>,
    #[serde(rename="stpId", skip_serializing_if = "Option::is_none")]
    pub stp_id: Option<String>,
    #[serde(rename="stpMode", skip_serializing_if = "Option::is_none")]
    pub stp_mode: Option<String>,
    #[serde(rename="expTime", skip_serializing_if = "Option::is_none")]
    pub exp_time: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

