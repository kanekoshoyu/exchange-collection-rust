#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WsUserNonFundingLedgerUpdate represents a WsUserNonFundingLedgerUpdate model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WsUserNonFundingLedgerUpdate {
    #[serde(rename="time")]
    pub time: i32,
    #[serde(rename="hash")]
    pub hash: String,
    #[serde(rename="delta")]
    pub delta: Box<WsDelta>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

