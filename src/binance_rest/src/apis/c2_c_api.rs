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


/// struct for typed errors of method [`sapi_v1_c2c_order_match_list_user_order_history_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1C2cOrderMatchListUserOrderHistoryGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}


/// - If startTimestamp and endTimestamp are not sent, the recent 30-day data will be returned. - The max interval between startTimestamp and endTimestamp is 30 days.  Weight(IP): 1
pub async fn sapi_v1_c2c_order_match_list_user_order_history_get(configuration: &configuration::Configuration, trade_type: &str, timestamp: i64, signature: &str, start_timestamp: Option<i64>, end_timestamp: Option<i64>, page: Option<i32>, rows: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1C2cOrderMatchListUserOrderHistoryGet200Response, Error<SapiV1C2cOrderMatchListUserOrderHistoryGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_trade_type = trade_type;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_start_timestamp = start_timestamp;
    let p_end_timestamp = end_timestamp;
    let p_page = page;
    let p_rows = rows;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/c2c/orderMatch/listUserOrderHistory", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("tradeType", &p_trade_type.to_string())]);
    if let Some(ref param_value) = p_start_timestamp {
        req_builder = req_builder.query(&[("startTimestamp", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_timestamp {
        req_builder = req_builder.query(&[("endTimestamp", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_rows {
        req_builder = req_builder.query(&[("rows", &param_value.to_string())]);
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
        let entity: Option<SapiV1C2cOrderMatchListUserOrderHistoryGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

