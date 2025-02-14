/*
 * Binance Public Spot API
 *
 * OpenAPI Specifications for the Binance Public Spot API  API documents:   - [https://github.com/binance/binance-spot-api-docs](https://github.com/binance/binance-spot-api-docs)   - [https://binance-docs.github.io/apidocs/spot/en](https://binance-docs.github.io/apidocs/spot/en)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`sapi_v1_rebate_tax_query_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1RebateTaxQueryGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}


/// - The max interval between startTime and endTime is 90 days. - If startTime and endTime are not sent, the recent 7 days' data will be returned. - The earliest startTime is supported on June 10, 2020  Weight(UID): 3000
pub async fn sapi_v1_rebate_tax_query_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, start_time: Option<i64>, end_time: Option<i64>, page: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1RebateTaxQueryGet200Response, Error<SapiV1RebateTaxQueryGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_start_time = start_time;
    let p_end_time = end_time;
    let p_page = page;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/rebate/taxQuery", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_start_time {
        req_builder = req_builder.query(&[("startTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_time {
        req_builder = req_builder.query(&[("endTime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_recv_window {
        req_builder = req_builder.query(&[("recvWindow", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("timestamp", &p_timestamp.to_string())]);
    req_builder = req_builder.query(&[("signature", &p_signature.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-MBX-APIKEY", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SapiV1RebateTaxQueryGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

