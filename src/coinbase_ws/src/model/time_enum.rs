#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TimeEnum represents a TimeEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TimeEnum {
    #[serde(rename="UNKNOWN_TIME_IN_FORCE")]
    UnknownTimeInForce,
    #[serde(rename="GOOD_UNTIL_CANELLED")]
    GoodUntilCanelled,
    #[serde(rename="GOOD_UNTIL_DATE_TIME")]
    GoodUntilDateTime,
    #[serde(rename="IMMIDIATE_OR_CANCEL")]
    ImmidiateOrCancel,
    #[serde(rename="FILL_OR_KILL")]
    FillOrKill,
}

