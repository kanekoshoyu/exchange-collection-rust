#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SlashStream represents a union of types: StreamControlResponse, Error, AggregateTradeStream, TradeStream, KlineStreamWithTimezoneOffset, KlineStreamWithUtc, IndividualSymbolMiniTicker, Vec<AllMarketMiniTicker>, IndividualSymbolTicker, Vec<AllMarketTicker>, IndividualSymbolRollingWindowStatistics, Vec<AllMarketRollingWindowStat>, IndividualSymbolBookTicker, AveragePrice, PartialBookDepth, DiffDepth
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum SlashStream {
    #[serde(rename="StreamControlResponse")]
    StreamControlResponse(StreamControlResponse),
    #[serde(rename="Error")]
    Error(Error),
    #[serde(rename="AggregateTradeStream")]
    AggregateTradeStream(AggregateTradeStream),
    #[serde(rename="TradeStream")]
    TradeStream(TradeStream),
    #[serde(rename="KlineStreamWithTimezoneOffset")]
    KlineStreamWithTimezoneOffset(KlineStreamWithTimezoneOffset),
    #[serde(rename="KlineStreamWithUtc")]
    KlineStreamWithUtc(KlineStreamWithUtc),
    #[serde(rename="IndividualSymbolMiniTicker")]
    IndividualSymbolMiniTicker(IndividualSymbolMiniTicker),
    #[serde(rename="AllMarketMiniTickers")]
    AllMarketMiniTickers(Vec<AllMarketMiniTicker>),
    #[serde(rename="IndividualSymbolTicker")]
    IndividualSymbolTicker(IndividualSymbolTicker),
    #[serde(rename="AllMarketTickers")]
    AllMarketTickers(Vec<AllMarketTicker>),
    #[serde(rename="IndividualSymbolRollingWindowStatistics")]
    IndividualSymbolRollingWindowStatistics(IndividualSymbolRollingWindowStatistics),
    #[serde(rename="AllMarketRollingWindowStatistics")]
    AllMarketRollingWindowStatistics(Vec<AllMarketRollingWindowStat>),
    #[serde(rename="IndividualSymbolBookTicker")]
    IndividualSymbolBookTicker(IndividualSymbolBookTicker),
    #[serde(rename="AveragePrice")]
    AveragePrice(AveragePrice),
    #[serde(rename="PartialBookDepth")]
    PartialBookDepth(PartialBookDepth),
    #[serde(rename="DiffDepth")]
    DiffDepth(DiffDepth),
}


