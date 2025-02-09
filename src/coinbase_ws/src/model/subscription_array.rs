#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SubscriptionArray represents a union of types: SubscriptionPayload, TradeSubscriptionPayload
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum SubscriptionArray {
    #[serde(rename="SubscriptionPayload")]
    SubscriptionPayload(SubscriptionPayload),
    #[serde(rename="TradeSubscriptionPayload")]
    TradeSubscriptionPayload(TradeSubscriptionPayload),
}


