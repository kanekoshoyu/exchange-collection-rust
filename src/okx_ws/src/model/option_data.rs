#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OptionData represents a OptionData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct OptionData {
    #[serde(rename="instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    #[serde(rename="instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(rename="uly", skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    #[serde(rename="delta", skip_serializing_if = "Option::is_none")]
    pub delta: Option<String>,
    #[serde(rename="gamma", skip_serializing_if = "Option::is_none")]
    pub gamma: Option<String>,
    #[serde(rename="vega", skip_serializing_if = "Option::is_none")]
    pub vega: Option<String>,
    #[serde(rename="theta", skip_serializing_if = "Option::is_none")]
    pub theta: Option<String>,
    #[serde(rename="deltaBS", skip_serializing_if = "Option::is_none")]
    pub delta_bs: Option<String>,
    #[serde(rename="gammaBS", skip_serializing_if = "Option::is_none")]
    pub gamma_bs: Option<String>,
    #[serde(rename="vegaBS", skip_serializing_if = "Option::is_none")]
    pub vega_bs: Option<String>,
    #[serde(rename="thetaBS", skip_serializing_if = "Option::is_none")]
    pub theta_bs: Option<String>,
    #[serde(rename="lever", skip_serializing_if = "Option::is_none")]
    pub lever: Option<String>,
    #[serde(rename="markVol", skip_serializing_if = "Option::is_none")]
    pub mark_vol: Option<String>,
    #[serde(rename="bidVol", skip_serializing_if = "Option::is_none")]
    pub bid_vol: Option<String>,
    #[serde(rename="askVol", skip_serializing_if = "Option::is_none")]
    pub ask_vol: Option<String>,
    #[serde(rename="realVol", skip_serializing_if = "Option::is_none")]
    pub real_vol: Option<String>,
    #[serde(rename="volLv", skip_serializing_if = "Option::is_none")]
    pub vol_lv: Option<String>,
    #[serde(rename="fwdPx", skip_serializing_if = "Option::is_none")]
    pub fwd_px: Option<String>,
    #[serde(rename="ts", skip_serializing_if = "Option::is_none")]
    pub ts: Option<String>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

