#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Union represents a union of types: DataItem, SubscriptionData, TradeData, BookData, FillData, OrderData, TriggerData, Account, FutureData, FuturesAccountData, FuturesPositionData, FuturesFillData, serde_json::Value
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Union {
    #[serde(rename="DataItem")]
    DataItem(DataItem),
    #[serde(rename="SubscriptionData")]
    SubscriptionData(SubscriptionData),
    #[serde(rename="TradeData")]
    TradeData(TradeData),
    #[serde(rename="BookData")]
    BookData(BookData),
    #[serde(rename="FillData")]
    FillData(FillData),
    #[serde(rename="OrderData")]
    OrderData(OrderData),
    #[serde(rename="TriggerData")]
    TriggerData(TriggerData),
    #[serde(rename="Account")]
    Account(Account),
    #[serde(rename="FutureData")]
    FutureData(FutureData),
    #[serde(rename="FuturesAccountData")]
    FuturesAccountData(FuturesAccountData),
    #[serde(rename="FuturesPositionData")]
    FuturesPositionData(FuturesPositionData),
    #[serde(rename="FuturesFillData")]
    FuturesFillData(FuturesFillData),
    #[serde(rename="Undefined")]
    Undefined(serde_json::Value),
}


