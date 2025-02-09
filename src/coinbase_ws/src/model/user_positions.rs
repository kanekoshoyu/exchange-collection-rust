#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// UserPositions represents a UserPositions model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct UserPositions {
    #[serde(rename="perpetual_futures_positions", skip_serializing_if = "Option::is_none")]
    pub perpetual_futures_positions: Option<Vec<PerpetualFuturesPosition>>,
    #[serde(rename="expiring_futures_positions", skip_serializing_if = "Option::is_none")]
    pub expiring_futures_positions: Option<Vec<ExpiringFuturesPosition>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

