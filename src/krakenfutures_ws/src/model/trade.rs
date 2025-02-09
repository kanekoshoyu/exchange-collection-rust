#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Trade represents a union of types: TickerResponse, TradeSnapshotResponse, TradeDeltaResponse, ErrorResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Trade {
    #[serde(rename="TickerResponse")]
    TickerResponse(TickerResponse),
    #[serde(rename="TradeSnapshotResponse")]
    TradeSnapshotResponse(TradeSnapshotResponse),
    #[serde(rename="TradeDeltaResponse")]
    TradeDeltaResponse(TradeDeltaResponse),
    #[serde(rename="ErrorResponse")]
    ErrorResponse(ErrorResponse),
}


