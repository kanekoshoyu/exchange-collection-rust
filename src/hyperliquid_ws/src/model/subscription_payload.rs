#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// SubscriptionPayload represents a SubscriptionPayload model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SubscriptionPayload {
    #[serde(rename="method", skip_serializing_if = "Option::is_none")]
    pub method: Option<Box<SubscriptionEnum>>,
    #[serde(rename="subscription", skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Box<Subscriptions>>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

