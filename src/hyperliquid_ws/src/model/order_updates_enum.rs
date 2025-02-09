#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OrderUpdatesEnum represents a OrderUpdatesEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum OrderUpdatesEnum {
    #[serde(rename="orderUpdates")]
    OrderUpdates,
}

