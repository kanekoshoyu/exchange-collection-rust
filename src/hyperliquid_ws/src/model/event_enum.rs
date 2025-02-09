#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// EventEnum represents a EventEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum EventEnum {
    #[serde(rename="userEvents")]
    UserEvents,
}

