#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// Subscriptions represents a union of types: AllMidsSubscription, NotificationSubscription, WebData2Subscription, CandleSubscription, L2BookSubscription, TradesSubscription, OrderUpdatesSubscription, UserEventsSubscription, UserFillsSubscription, UserFundingsSubscription, WsUserNonFundingLedgerUpdate
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Subscriptions {
    #[serde(rename="AllMidsSubscription")]
    AllMidsSubscription(AllMidsSubscription),
    #[serde(rename="NotificationSubscription")]
    NotificationSubscription(NotificationSubscription),
    #[serde(rename="WebData2Subscription")]
    WebData2Subscription(WebData2Subscription),
    #[serde(rename="CandleSubscription")]
    CandleSubscription(CandleSubscription),
    #[serde(rename="L2BookSubscription")]
    L2BookSubscription(L2BookSubscription),
    #[serde(rename="TradesSubscription")]
    TradesSubscription(TradesSubscription),
    #[serde(rename="OrderUpdatesSubscription")]
    OrderUpdatesSubscription(OrderUpdatesSubscription),
    #[serde(rename="UserEventsSubscription")]
    UserEventsSubscription(UserEventsSubscription),
    #[serde(rename="UserFillsSubscription")]
    UserFillsSubscription(UserFillsSubscription),
    #[serde(rename="UserFundingsSubscription")]
    UserFundingsSubscription(UserFundingsSubscription),
    #[serde(rename="WsUserNonFundingLedgerUpdate")]
    WsUserNonFundingLedgerUpdate(WsUserNonFundingLedgerUpdate),
}


