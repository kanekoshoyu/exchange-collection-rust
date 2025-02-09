#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// EventEnum represents a EventEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum EventEnum {
    #[serde(rename="subscribe")]
    Subscribe,
    #[serde(rename="unsubscribe")]
    Unsubscribe,
    #[serde(rename="subscribed_failed")]
    SubscribedFailed,
    #[serde(rename="unsubscribed_failed")]
    UnsubscribedFailed,
}

