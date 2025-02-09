#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AccountEnum represents a AccountEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum AccountEnum {
    #[serde(rename="outboundAccountInfo")]
    OutboundAccountInfo,
    #[serde(rename="outboundCustodyAccountInfo")]
    OutboundCustodyAccountInfo,
    #[serde(rename="outboundFiatAccountInfo")]
    OutboundFiatAccountInfo,
}

