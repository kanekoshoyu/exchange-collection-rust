#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TimeEnum represents a TimeEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TimeEnum {
    #[serde(rename="0")]
    Number0,
    #[serde(rename="1")]
    Number1,
    #[serde(rename="2")]
    Number2,
    #[serde(rename="3")]
    Number3,
    #[serde(rename="4")]
    Number4,
    #[serde(rename="5")]
    Number5,
    #[serde(rename="6")]
    Number6,
    #[serde(rename="7")]
    Number7,
}

