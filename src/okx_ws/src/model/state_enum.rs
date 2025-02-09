#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// StateEnum represents a StateEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum StateEnum {
    #[serde(rename="canceled")]
    Canceled,
    #[serde(rename="live")]
    Live,
    #[serde(rename="partially_filled")]
    PartiallyFilled,
    #[serde(rename="filled")]
    Filled,
    #[serde(rename="mmp_canceled")]
    MmpCanceled,
}

