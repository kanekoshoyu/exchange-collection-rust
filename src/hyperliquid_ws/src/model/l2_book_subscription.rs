#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// L2BookSubscription represents a L2BookSubscription model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct L2BookSubscription {
    #[serde(rename="type")]
    pub reserved_type: Box<L2Enum>,
    #[serde(rename="coin")]
    pub coin: String,
    #[serde(rename="additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<std::collections::HashMap<String, serde_json::Value>>,
}

