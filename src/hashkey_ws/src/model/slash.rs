#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Slash represents a union of types: PongPayload, KlinePayload, RealtimePayload, TradePayload, DepthPayload, Vec<AccountSchema>, Vec<OrderSchema>, Vec<TicketSchema>
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Slash {
    #[serde(rename="PongPayload")]
    PongPayload(PongPayload),
    #[serde(rename="KlinePayload")]
    KlinePayload(KlinePayload),
    #[serde(rename="RealtimePayload")]
    RealtimePayload(RealtimePayload),
    #[serde(rename="TradePayload")]
    TradePayload(TradePayload),
    #[serde(rename="DepthPayload")]
    DepthPayload(DepthPayload),
    #[serde(rename="AnonymousSchema99")]
    AnonymousSchema99(Vec<AccountSchema>),
    #[serde(rename="AnonymousSchema108")]
    AnonymousSchema108(Vec<OrderSchema>),
    #[serde(rename="AnonymousSchema134")]
    AnonymousSchema134(Vec<TicketSchema>),
}


