#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ProductEnum represents a ProductEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ProductEnum {
    #[serde(rename="UNKNOWN_PRODUCT_TYPE")]
    UnknownProductType,
    #[serde(rename="SPOT")]
    Spot,
    #[serde(rename="FUTURE")]
    Future,
}

