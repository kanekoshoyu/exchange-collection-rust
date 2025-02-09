#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SubscriptionData represents a union of types: SubscriptionDataObject, Vec<String>
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum SubscriptionData {
    #[serde(rename="SubscriptionDataObject")]
    SubscriptionDataObject(SubscriptionDataObject),
    #[serde(rename="SubscriptionDataArray")]
    SubscriptionDataArray(Vec<String>),
}


