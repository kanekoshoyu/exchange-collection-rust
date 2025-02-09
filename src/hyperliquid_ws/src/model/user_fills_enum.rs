#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// UserFillsEnum represents a UserFillsEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum UserFillsEnum {
    #[serde(rename="userFills")]
    UserFills,
}

