#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// BalancePushData represents a BalancePushData model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BalancePushData {
    #[serde(rename="pTime", skip_serializing_if = "Option::is_none")]
    pub p_time: Option<String>,
    #[serde(rename="eventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<Box<EventEnum>>,
    #[serde(rename="balData", skip_serializing_if = "Option::is_none")]
    pub bal_data: Option<Box<BalanceData>>,
    #[serde(rename="posData", skip_serializing_if = "Option::is_none")]
    pub pos_data: Option<Box<PositionData>>,
    #[serde(rename="trades", skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<TradeData2>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

