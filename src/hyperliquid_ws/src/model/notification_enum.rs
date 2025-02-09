#[allow(unused)]
use super::*;
use serde::{Deserialize, Serialize};

/// NotificationEnum represents a NotificationEnum model.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum NotificationEnum {
    #[serde(rename="notification")]
    Notification,
}

