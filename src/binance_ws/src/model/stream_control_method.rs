#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// StreamControlMethod represents a StreamControlMethod model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum StreamControlMethod {
    #[serde(rename="SUBSCRIBE")]
    Subscribe,
    #[serde(rename="UNSUBSCRIBE")]
    Unsubscribe,
    #[serde(rename="LIST_SUBSCRIPTIONS")]
    ListSubscriptions,
    #[serde(rename="SET_PROPERTY")]
    SetProperty,
}

