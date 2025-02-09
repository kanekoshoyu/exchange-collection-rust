#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Ticker represents a union of types: TickerResponse, TickerSnapshotResponse, ErrorResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Ticker {
    #[serde(rename="TickerResponse")]
    TickerResponse(TickerResponse),
    #[serde(rename="TickerSnapshotResponse")]
    TickerSnapshotResponse(TickerSnapshotResponse),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(ErrorResponse),
}


