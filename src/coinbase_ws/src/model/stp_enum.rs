#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// StpEnum represents a StpEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum StpEnum {
    #[serde(rename="STP")]
    Stp,
    #[serde(rename="modify_order")]
    ModifyOrder,
    #[serde(rename="other_reason")]
    OtherReason,
}

