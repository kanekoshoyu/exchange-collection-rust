#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// ChannelOrProductIds represents a union of types: ChannelEnum, ProductIds
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ChannelOrProductIds {
    #[serde(rename="ChannelEnum")]
    ChannelEnum(ChannelEnum),
    #[serde(rename="ProductIds")]
    ProductIds(ProductIds),
}


