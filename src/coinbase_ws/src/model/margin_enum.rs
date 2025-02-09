#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// MarginEnum represents a MarginEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum MarginEnum {
    #[serde(rename="FCM_MARGIN_WINDOW_TYPE_UNSPECIFIED")]
    FcmMarginWindowTypeUnspecified,
    #[serde(rename="FCM_MARGIN_WINDOW_TYPE_OVERNIGHT")]
    FcmMarginWindowTypeOvernight,
    #[serde(rename="FCM_MARGIN_WINDOW_TYPE_WEEKEND")]
    FcmMarginWindowTypeWeekend,
    #[serde(rename="FCM_MARGIN_WINDOW_TYPE_INTRADAY")]
    FcmMarginWindowTypeIntraday,
    #[serde(rename="FCM_MARGIN_WINDOW_TYPE_TRANSITION")]
    FcmMarginWindowTypeTransition,
}

