#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// WebDataEnum represents a WebDataEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum WebDataEnum {
    #[serde(rename="webData2")]
    WebData2,
}

