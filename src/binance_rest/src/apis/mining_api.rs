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


/// struct for typed errors of method [`sapi_v1_mining_hash_transfer_config_cancel_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningHashTransferConfigCancelPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_mining_hash_transfer_config_details_list_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningHashTransferConfigDetailsListGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_mining_hash_transfer_config_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningHashTransferConfigPostError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_mining_hash_transfer_profit_details_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningHashTransferProfitDetailsGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_mining_payment_list_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningPaymentListGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_mining_payment_other_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningPaymentOtherGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_mining_payment_uid_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningPaymentUidGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_mining_pub_algo_list_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningPubAlgoListGetError {
    Status400(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_mining_pub_coin_list_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningPubCoinListGetError {
    Status400(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_mining_statistics_user_list_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningStatisticsUserListGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_mining_statistics_user_status_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningStatisticsUserStatusGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_mining_worker_detail_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningWorkerDetailGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sapi_v1_mining_worker_list_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SapiV1MiningWorkerListGetError {
    Status400(models::Error),
    Status401(models::Error),
    UnknownValue(serde_json::Value),
}


/// Weight(IP): 5
pub async fn sapi_v1_mining_hash_transfer_config_cancel_post(configuration: &configuration::Configuration, config_id: &str, user_name: &str, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1MiningHashTransferConfigCancelPost200Response, Error<SapiV1MiningHashTransferConfigCancelPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_config_id = config_id;
    let p_user_name = user_name;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/mining/hash-transfer/config/cancel", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    req_builder = req_builder.query(&[("configId", &p_config_id.to_string())]);
    req_builder = req_builder.query(&[("userName", &p_user_name.to_string())]);
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
        let entity: Option<SapiV1MiningHashTransferConfigCancelPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 5
pub async fn sapi_v1_mining_hash_transfer_config_details_list_get(configuration: &configuration::Configuration, timestamp: i64, signature: &str, page_index: Option<i32>, page_size: Option<&str>, recv_window: Option<i64>) -> Result<models::SapiV1MiningHashTransferConfigDetailsListGet200Response, Error<SapiV1MiningHashTransferConfigDetailsListGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_page_index = page_index;
    let p_page_size = page_size;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/mining/hash-transfer/config/details/list", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page_index {
        req_builder = req_builder.query(&[("pageIndex", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("pageSize", &param_value.to_string())]);
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
        let entity: Option<SapiV1MiningHashTransferConfigDetailsListGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 5
pub async fn sapi_v1_mining_hash_transfer_config_post(configuration: &configuration::Configuration, user_name: &str, algo: &str, to_pool_user: &str, hash_rate: &str, timestamp: i64, signature: &str, start_date: Option<&str>, end_date: Option<&str>, recv_window: Option<i64>) -> Result<models::SapiV1MiningHashTransferConfigPost200Response, Error<SapiV1MiningHashTransferConfigPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_user_name = user_name;
    let p_algo = algo;
    let p_to_pool_user = to_pool_user;
    let p_hash_rate = hash_rate;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_start_date = start_date;
    let p_end_date = end_date;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/mining/hash-transfer/config", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    req_builder = req_builder.query(&[("userName", &p_user_name.to_string())]);
    req_builder = req_builder.query(&[("algo", &p_algo.to_string())]);
    if let Some(ref param_value) = p_start_date {
        req_builder = req_builder.query(&[("startDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_date {
        req_builder = req_builder.query(&[("endDate", &param_value.to_string())]);
    }
    req_builder = req_builder.query(&[("toPoolUser", &p_to_pool_user.to_string())]);
    req_builder = req_builder.query(&[("hashRate", &p_hash_rate.to_string())]);
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
        let entity: Option<SapiV1MiningHashTransferConfigPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 5
pub async fn sapi_v1_mining_hash_transfer_profit_details_get(configuration: &configuration::Configuration, config_id: &str, user_name: &str, timestamp: i64, signature: &str, page_index: Option<i32>, page_size: Option<&str>, recv_window: Option<i64>) -> Result<models::SapiV1MiningHashTransferProfitDetailsGet200Response, Error<SapiV1MiningHashTransferProfitDetailsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_config_id = config_id;
    let p_user_name = user_name;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_page_index = page_index;
    let p_page_size = page_size;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/mining/hash-transfer/profit/details", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("configId", &p_config_id.to_string())]);
    req_builder = req_builder.query(&[("userName", &p_user_name.to_string())]);
    if let Some(ref param_value) = p_page_index {
        req_builder = req_builder.query(&[("pageIndex", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("pageSize", &param_value.to_string())]);
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
        let entity: Option<SapiV1MiningHashTransferProfitDetailsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 5
pub async fn sapi_v1_mining_payment_list_get(configuration: &configuration::Configuration, algo: &str, user_name: &str, timestamp: i64, signature: &str, coin: Option<&str>, start_date: Option<&str>, end_date: Option<&str>, page_index: Option<i32>, page_size: Option<&str>, recv_window: Option<i64>) -> Result<models::SapiV1MiningPaymentListGet200Response, Error<SapiV1MiningPaymentListGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_algo = algo;
    let p_user_name = user_name;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_coin = coin;
    let p_start_date = start_date;
    let p_end_date = end_date;
    let p_page_index = page_index;
    let p_page_size = page_size;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/mining/payment/list", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("algo", &p_algo.to_string())]);
    req_builder = req_builder.query(&[("userName", &p_user_name.to_string())]);
    if let Some(ref param_value) = p_coin {
        req_builder = req_builder.query(&[("coin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_start_date {
        req_builder = req_builder.query(&[("startDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_date {
        req_builder = req_builder.query(&[("endDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_index {
        req_builder = req_builder.query(&[("pageIndex", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("pageSize", &param_value.to_string())]);
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
        let entity: Option<SapiV1MiningPaymentListGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 5
pub async fn sapi_v1_mining_payment_other_get(configuration: &configuration::Configuration, algo: &str, user_name: &str, timestamp: i64, signature: &str, coin: Option<&str>, start_date: Option<&str>, end_date: Option<&str>, page_index: Option<i32>, page_size: Option<&str>, recv_window: Option<i64>) -> Result<models::SapiV1MiningPaymentOtherGet200Response, Error<SapiV1MiningPaymentOtherGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_algo = algo;
    let p_user_name = user_name;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_coin = coin;
    let p_start_date = start_date;
    let p_end_date = end_date;
    let p_page_index = page_index;
    let p_page_size = page_size;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/mining/payment/other", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("algo", &p_algo.to_string())]);
    req_builder = req_builder.query(&[("userName", &p_user_name.to_string())]);
    if let Some(ref param_value) = p_coin {
        req_builder = req_builder.query(&[("coin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_start_date {
        req_builder = req_builder.query(&[("startDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_date {
        req_builder = req_builder.query(&[("endDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_index {
        req_builder = req_builder.query(&[("pageIndex", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("pageSize", &param_value.to_string())]);
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
        let entity: Option<SapiV1MiningPaymentOtherGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 5
pub async fn sapi_v1_mining_payment_uid_get(configuration: &configuration::Configuration, algo: &str, timestamp: i64, signature: &str, start_date: Option<&str>, end_date: Option<&str>, page_index: Option<i32>, page_size: Option<&str>, recv_window: Option<i64>) -> Result<models::SapiV1MiningPaymentUidGet200Response, Error<SapiV1MiningPaymentUidGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_algo = algo;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_start_date = start_date;
    let p_end_date = end_date;
    let p_page_index = page_index;
    let p_page_size = page_size;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/mining/payment/uid", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("algo", &p_algo.to_string())]);
    if let Some(ref param_value) = p_start_date {
        req_builder = req_builder.query(&[("startDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_date {
        req_builder = req_builder.query(&[("endDate", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_index {
        req_builder = req_builder.query(&[("pageIndex", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("pageSize", &param_value.to_string())]);
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
        let entity: Option<SapiV1MiningPaymentUidGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 1
pub async fn sapi_v1_mining_pub_algo_list_get(configuration: &configuration::Configuration, ) -> Result<models::SapiV1MiningPubAlgoListGet200Response, Error<SapiV1MiningPubAlgoListGetError>> {

    let uri_str = format!("{}/sapi/v1/mining/pub/algoList", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
        let entity: Option<SapiV1MiningPubAlgoListGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 1
pub async fn sapi_v1_mining_pub_coin_list_get(configuration: &configuration::Configuration, ) -> Result<models::SapiV1MiningPubCoinListGet200Response, Error<SapiV1MiningPubCoinListGetError>> {

    let uri_str = format!("{}/sapi/v1/mining/pub/coinList", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
        let entity: Option<SapiV1MiningPubCoinListGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 5
pub async fn sapi_v1_mining_statistics_user_list_get(configuration: &configuration::Configuration, algo: &str, user_name: &str, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1MiningStatisticsUserListGet200Response, Error<SapiV1MiningStatisticsUserListGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_algo = algo;
    let p_user_name = user_name;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/mining/statistics/user/list", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("algo", &p_algo.to_string())]);
    req_builder = req_builder.query(&[("userName", &p_user_name.to_string())]);
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
        let entity: Option<SapiV1MiningStatisticsUserListGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 5
pub async fn sapi_v1_mining_statistics_user_status_get(configuration: &configuration::Configuration, algo: &str, user_name: &str, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1MiningStatisticsUserStatusGet200Response, Error<SapiV1MiningStatisticsUserStatusGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_algo = algo;
    let p_user_name = user_name;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/mining/statistics/user/status", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("algo", &p_algo.to_string())]);
    req_builder = req_builder.query(&[("userName", &p_user_name.to_string())]);
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
        let entity: Option<SapiV1MiningStatisticsUserStatusGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 5
pub async fn sapi_v1_mining_worker_detail_get(configuration: &configuration::Configuration, algo: &str, user_name: &str, worker_name: &str, timestamp: i64, signature: &str, recv_window: Option<i64>) -> Result<models::SapiV1MiningWorkerDetailGet200Response, Error<SapiV1MiningWorkerDetailGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_algo = algo;
    let p_user_name = user_name;
    let p_worker_name = worker_name;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/mining/worker/detail", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("algo", &p_algo.to_string())]);
    req_builder = req_builder.query(&[("userName", &p_user_name.to_string())]);
    req_builder = req_builder.query(&[("workerName", &p_worker_name.to_string())]);
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
        let entity: Option<SapiV1MiningWorkerDetailGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Weight(IP): 5
pub async fn sapi_v1_mining_worker_list_get(configuration: &configuration::Configuration, algo: &str, user_name: &str, timestamp: i64, signature: &str, page_index: Option<i32>, sort: Option<i32>, sort_column: Option<i32>, worker_status: Option<i32>, recv_window: Option<i64>) -> Result<models::SapiV1MiningWorkerListGet200Response, Error<SapiV1MiningWorkerListGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_algo = algo;
    let p_user_name = user_name;
    let p_timestamp = timestamp;
    let p_signature = signature;
    let p_page_index = page_index;
    let p_sort = sort;
    let p_sort_column = sort_column;
    let p_worker_status = worker_status;
    let p_recv_window = recv_window;

    let uri_str = format!("{}/sapi/v1/mining/worker/list", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("algo", &p_algo.to_string())]);
    req_builder = req_builder.query(&[("userName", &p_user_name.to_string())]);
    if let Some(ref param_value) = p_page_index {
        req_builder = req_builder.query(&[("pageIndex", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort_column {
        req_builder = req_builder.query(&[("sortColumn", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_worker_status {
        req_builder = req_builder.query(&[("workerStatus", &param_value.to_string())]);
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
        let entity: Option<SapiV1MiningWorkerListGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

