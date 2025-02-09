#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// CandleEnum represents a CandleEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum CandleEnum {
    #[serde(rename="candle")]
    Candle,
}

