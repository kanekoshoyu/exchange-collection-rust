#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SignalEnum represents a SignalEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum SignalEnum {
    #[serde(rename="last")]
    Last,
    #[serde(rename="mark")]
    Mark,
}

