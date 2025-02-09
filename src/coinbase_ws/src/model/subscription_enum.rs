#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SubscriptionEnum represents a SubscriptionEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum SubscriptionEnum {
    #[serde(rename="subscribe")]
    Subscribe,
    #[serde(rename="unsubscribed")]
    Unsubscribed,
}

