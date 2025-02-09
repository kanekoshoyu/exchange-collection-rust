#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// TradeChannelEnum represents a TradeChannelEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TradeChannelEnum {
    #[serde(rename="level2")]
    Level2,
    #[serde(rename="heartbeats")]
    Heartbeats,
    #[serde(rename="candles")]
    Candles,
    #[serde(rename="market_trades")]
    MarketTrades,
    #[serde(rename="status")]
    Status,
    #[serde(rename="ticker")]
    Ticker,
    #[serde(rename="ticker_batch")]
    TickerBatch,
    #[serde(rename="l2_data")]
    L2Data,
    #[serde(rename="user")]
    User,
    #[serde(rename="futures_balance_summary")]
    FuturesBalanceSummary,
}

