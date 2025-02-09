#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SubscriptionDataObject represents a SubscriptionDataObject model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionDataObject {
    #[serde(rename="0", skip_serializing_if = "Option::is_none")]
    pub number_0: Option<String>,
    #[serde(rename="1", skip_serializing_if = "Option::is_none")]
    pub number_1: Option<String>,
    #[serde(rename="2", skip_serializing_if = "Option::is_none")]
    pub number_2: Option<String>,
    #[serde(rename="3", skip_serializing_if = "Option::is_none")]
    pub number_3: Option<String>,
    #[serde(rename="4", skip_serializing_if = "Option::is_none")]
    pub number_4: Option<String>,
    #[serde(rename="5", skip_serializing_if = "Option::is_none")]
    pub number_5: Option<String>,
    #[serde(rename="6", skip_serializing_if = "Option::is_none")]
    pub number_6: Option<String>,
    #[serde(rename="7", skip_serializing_if = "Option::is_none")]
    pub number_7: Option<String>,
}

