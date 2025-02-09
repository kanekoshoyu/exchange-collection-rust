#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ChannelEnum represents a ChannelEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ChannelEnum {
    #[serde(rename="level2")]
    Level2,
    #[serde(rename="heartbeat")]
    Heartbeat,
    #[serde(rename="status")]
    Status,
    #[serde(rename="aution")]
    Aution,
    #[serde(rename="ticker")]
    Ticker,
    #[serde(rename="matches")]
    Matches,
    #[serde(rename="rfq-_matches")]
    RfqMinusMatches,
    #[serde(rename="ticker_batch")]
    TickerBatch,
    #[serde(rename="full")]
    Full,
}

