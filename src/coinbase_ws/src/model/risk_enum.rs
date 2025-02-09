#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// RiskEnum represents a RiskEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum RiskEnum {
    #[serde(rename="UNKNOWN_RISK_MANAGEMENT_TYPE")]
    UnknownRiskManagementType,
    #[serde(rename="MANAGED_BY_FCM")]
    ManagedByFcm,
    #[serde(rename="MANAGED_BY_VENUE")]
    ManagedByVenue,
}

