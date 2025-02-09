/*
 * MEXC Crypto Exchange
 *
 * Welcome to mexc API document! 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RebateDetails {
    /// Current page number of results.
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    /// Total number of rebate records available.
    #[serde(rename = "totalRecords", skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i32>,
    /// Total number of pages for the query.
    #[serde(rename = "totalPageNum", skip_serializing_if = "Option::is_none")]
    pub total_page_num: Option<i32>,
    /// List of detailed rebate records.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::RebateDetailsDataInner>>,
}

impl RebateDetails {
    pub fn new() -> RebateDetails {
        RebateDetails {
            page: None,
            total_records: None,
            total_page_num: None,
            data: None,
        }
    }
}

