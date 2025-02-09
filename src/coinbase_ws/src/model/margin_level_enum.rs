#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MarginLevelEnum represents a MarginLevelEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum MarginLevelEnum {
    #[serde(rename="MARGIN_LEVEL_TYPE_UNSPECIFIED")]
    MarginLevelTypeUnspecified,
    #[serde(rename="MARGIN_LEVEL_TYPE_BASE")]
    MarginLevelTypeBase,
    #[serde(rename="MARGIN_LEVEL_TYPE_WARNING")]
    MarginLevelTypeWarning,
    #[serde(rename="MARGIN_LEVEL_TYPE_DANGER")]
    MarginLevelTypeDanger,
    #[serde(rename="MARGIN_LEVEL_TYPE_LIQUIDATION")]
    MarginLevelTypeLiquidation,
}

