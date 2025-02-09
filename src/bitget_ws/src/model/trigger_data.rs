#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TriggerData represents a TriggerData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TriggerData {
    #[serde(rename="instId")]
    pub inst_id: String,
    #[serde(rename="orderId")]
    pub order_id: String,
    #[serde(rename="clientOid")]
    pub client_oid: String,
    #[serde(rename="triggerPrice")]
    pub trigger_price: String,
    #[serde(rename="triggerType")]
    pub trigger_type: String,
    #[serde(rename="planType")]
    pub plan_type: String,
    #[serde(rename="price")]
    pub price: String,
    #[serde(rename="size")]
    pub size: String,
    #[serde(rename="actualSize")]
    pub actual_size: String,
    #[serde(rename="orderType")]
    pub order_type: String,
    #[serde(rename="side")]
    pub side: String,
    #[serde(rename="status")]
    pub status: String,
    #[serde(rename="executePrice")]
    pub execute_price: String,
    #[serde(rename="enterPointSource")]
    pub enter_point_source: String,
    #[serde(rename="cTime")]
    pub c_time: String,
    #[serde(rename="uTime")]
    pub u_time: String,
    #[serde(rename="stpMode")]
    pub stp_mode: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

