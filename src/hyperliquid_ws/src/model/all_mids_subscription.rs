#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AllMidsSubscription represents a AllMidsSubscription model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AllMidsSubscription {
    #[serde(rename="type")]
    pub reserved_type: Box<AllMidsEnum>,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

