#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WsLedgerLiquidation represents a WsLedgerLiquidation model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct WsLedgerLiquidation {
    #[serde(rename="type")]
    pub reserved_type: Box<LedgerTypeEnum>,
    #[serde(rename="accountValue")]
    pub account_value: f64,
    #[serde(rename="leverageType")]
    pub leverage_type: Box<LeverageEnum>,
    #[serde(rename="liquidatedPositions")]
    pub liquidated_positions: Vec<LiquidatedPosition>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

