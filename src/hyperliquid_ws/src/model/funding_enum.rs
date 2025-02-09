#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// FundingEnum represents a FundingEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum FundingEnum {
    #[serde(rename="userFundings")]
    UserFundings,
}

