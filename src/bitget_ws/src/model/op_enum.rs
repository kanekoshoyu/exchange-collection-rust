#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// OpEnum represents a OpEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum OpEnum {
    #[serde(rename="subscribe")]
    Subscribe,
    #[serde(rename="snapshot")]
    Snapshot,
    #[serde(rename="login")]
    Login,
}

