#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// PayloadUnion represents a union of types: String, OrderRequest, OrderCancel, Vec<OrderCancel>, OrderEvents, TradeHistoryRequest, ChatRequest
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PayloadUnion {
    #[serde(rename="StringValue")]
    StringValue(String),
    #[serde(rename="OrderRequest")]
    OrderRequest(OrderRequest),
    #[serde(rename="OrderCancel")]
    OrderCancel(OrderCancel),
    #[serde(rename="BulkCancel")]
    BulkCancel(Vec<OrderCancel>),
    #[serde(rename="OrderEvents")]
    OrderEvents(OrderEvents),
    #[serde(rename="TradeHistoryRequest")]
    TradeHistoryRequest(TradeHistoryRequest),
    #[serde(rename="ChatRequest")]
    ChatRequest(ChatRequest),
}


