/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SapiV1ManagedSubaccountInfoGet200ResponseManagerSubUserInfoVoListInner {
    #[serde(rename = "rootUserId")]
    pub root_user_id: i64,
    #[serde(rename = "managersubUserId")]
    pub managersub_user_id: i64,
    #[serde(rename = "bindParentUserId")]
    pub bind_parent_user_id: i64,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "insertTimeStamp")]
    pub insert_time_stamp: i64,
    #[serde(rename = "bindParentEmail")]
    pub bind_parent_email: String,
    #[serde(rename = "isSubUserEnabled")]
    pub is_sub_user_enabled: bool,
    #[serde(rename = "isUserActive")]
    pub is_user_active: bool,
    #[serde(rename = "isMarginEnabled")]
    pub is_margin_enabled: bool,
    #[serde(rename = "isFutureEnabled")]
    pub is_future_enabled: bool,
    #[serde(rename = "isSignedLVTRiskAgreement")]
    pub is_signed_lvt_risk_agreement: bool,
}

impl SapiV1ManagedSubaccountInfoGet200ResponseManagerSubUserInfoVoListInner {
    pub fn new(root_user_id: i64, managersub_user_id: i64, bind_parent_user_id: i64, insert_time_stamp: i64, bind_parent_email: String, is_sub_user_enabled: bool, is_user_active: bool, is_margin_enabled: bool, is_future_enabled: bool, is_signed_lvt_risk_agreement: bool) -> SapiV1ManagedSubaccountInfoGet200ResponseManagerSubUserInfoVoListInner {
        SapiV1ManagedSubaccountInfoGet200ResponseManagerSubUserInfoVoListInner {
            root_user_id,
            managersub_user_id,
            bind_parent_user_id,
            email: None,
            insert_time_stamp,
            bind_parent_email,
            is_sub_user_enabled,
            is_user_active,
            is_margin_enabled,
            is_future_enabled,
            is_signed_lvt_risk_agreement,
        }
    }
}

