#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TickerLite represents a union of types: TickerResponse, TickerLiteSnapshotResponse, ErrorResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum TickerLite {
    #[serde(rename="TickerResponse")]
    TickerResponse(TickerResponse),
    #[serde(rename="TickerLiteSnapshotResponse")]
    TickerLiteSnapshotResponse(TickerLiteSnapshotResponse),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(ErrorResponse),
}


