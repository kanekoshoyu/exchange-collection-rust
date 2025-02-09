#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// StatusEnum represents a StatusEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum StatusEnum {
    #[serde(rename="PENDING")]
    Pending,
    #[serde(rename="OPEN")]
    Open,
    #[serde(rename="FILLED")]
    Filled,
    #[serde(rename="CANCELLED")]
    Cancelled,
    #[serde(rename="EXPIRED")]
    Expired,
    #[serde(rename="FAILED")]
    Failed,
}

