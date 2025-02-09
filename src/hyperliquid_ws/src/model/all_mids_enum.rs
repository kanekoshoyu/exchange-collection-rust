#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// AllMidsEnum represents a AllMidsEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum AllMidsEnum {
    #[serde(rename="allMids")]
    AllMids,
}

