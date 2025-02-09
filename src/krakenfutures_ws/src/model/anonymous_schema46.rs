#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AnonymousSchema46 represents a union of types: HoldingWalletResponse, MultiCollateralResponse, SingleCollateralResponse
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum AnonymousSchema46 {
    #[serde(rename="HoldingWalletResponse")]
    HoldingWalletResponse(HoldingWalletResponse),
    #[serde(rename="MultiCollateralResponse")]
    MultiCollateralResponse(MultiCollateralResponse),
    #[serde(rename="SingleCollateralResponse")]
    SingleCollateralResponse(SingleCollateralResponse),
}


