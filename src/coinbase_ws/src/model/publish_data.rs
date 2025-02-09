#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PublishData represents a union of types: serde_json::Value, HeartbeatPayload, StatusPayload, AuctionPayload, RfqMatchesPayload, TickerMessage, LimitOrderMessage, MarketOrderMessage, OpenOrderMessage, DoneOrders, MatchTakerMessage, MatchMakerMessage, StpOrderMessage, StopOrderMessage, serde_json::Value, serde_json::Value, serde_json::Value, serde_json::Value, TradeHeartbeat, TradeCandles, TradeMarkets, TradeStatus, TradeTicker, TradeL2Data, UserData, FutureBalanceSummary
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PublishData {
    #[serde(rename="AnonymousSchema1")]
    AnonymousSchema1(serde_json::Value),
    #[serde(rename="HeartbeatPayload")]
    HeartbeatPayload(HeartbeatPayload),
    #[serde(rename="StatusPayload")]
    StatusPayload(StatusPayload),
    #[serde(rename="AuctionPayload")]
    AuctionPayload(AuctionPayload),
    #[serde(rename="RfqMatchesPayload")]
    RfqMatchesPayload(RfqMatchesPayload),
    #[serde(rename="TickerMessage")]
    TickerMessage(TickerMessage),
    #[serde(rename="LimitOrderMessage")]
    LimitOrderMessage(LimitOrderMessage),
    #[serde(rename="MarketOrderMessage")]
    MarketOrderMessage(MarketOrderMessage),
    #[serde(rename="OpenOrderMessage")]
    OpenOrderMessage(OpenOrderMessage),
    #[serde(rename="DoneOrders")]
    DoneOrders(DoneOrders),
    #[serde(rename="MatchTakerMessage")]
    MatchTakerMessage(MatchTakerMessage),
    #[serde(rename="MatchMakerMessage")]
    MatchMakerMessage(MatchMakerMessage),
    #[serde(rename="StpOrderMessage")]
    StpOrderMessage(StpOrderMessage),
    #[serde(rename="StopOrderMessage")]
    StopOrderMessage(StopOrderMessage),
    #[serde(rename="OrderBookSnapshot")]
    OrderBookSnapshot(serde_json::Value),
    #[serde(rename="OrderBookUpdate")]
    OrderBookUpdate(serde_json::Value),
    #[serde(rename="MultipleArray")]
    MultipleArray(serde_json::Value),
    #[serde(rename="BalancePayload")]
    BalancePayload(serde_json::Value),
    #[serde(rename="TradeHeartbeat")]
    TradeHeartbeat(TradeHeartbeat),
    #[serde(rename="TradeCandles")]
    TradeCandles(TradeCandles),
    #[serde(rename="TradeMarkets")]
    TradeMarkets(TradeMarkets),
    #[serde(rename="TradeStatus")]
    TradeStatus(TradeStatus),
    #[serde(rename="TradeTicker")]
    TradeTicker(TradeTicker),
    #[serde(rename="TradeL2Data")]
    TradeL2Data(TradeL2Data),
    #[serde(rename="UserData")]
    UserData(UserData),
    #[serde(rename="FutureBalanceSummary")]
    FutureBalanceSummary(FutureBalanceSummary),
}


