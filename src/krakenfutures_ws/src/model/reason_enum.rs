#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ReasonEnum represents a ReasonEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ReasonEnum {
    #[serde(rename="new_placed_order_by_user")]
    NewPlacedOrderByUser,
    #[serde(rename="canceled_by_user")]
    CanceledByUser,
    #[serde(rename="other_reasons")]
    OtherReasons,
}

