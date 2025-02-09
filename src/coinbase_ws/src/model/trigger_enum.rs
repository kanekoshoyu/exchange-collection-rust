#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TriggerEnum represents a TriggerEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TriggerEnum {
    #[serde(rename="UNKNOWN_TRIGGER_STATUS")]
    UnknownTriggerStatus,
    #[serde(rename="INVALID_ORDER_TYPE")]
    InvalidOrderType,
    #[serde(rename="STOP_PENDING")]
    StopPending,
    #[serde(rename="STOP_TRIGGERED")]
    StopTriggered,
}

