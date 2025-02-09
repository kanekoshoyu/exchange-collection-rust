/*
 * OKX API
 *
 * Welcome to OKX Developer document!   excluded below endpoints as they are if you need them please add and commit to https://github.com/kanekoshoyu/exchange-collection): - Trading Account (this might become needed, will add when we need it) - Block Trading - Financial Producer - Affiliate - Status - Announcement 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`market_sprd_candles_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketSprdCandlesGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`market_sprd_history_candles_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketSprdHistoryCandlesGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`market_sprd_ticker_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarketSprdTickerGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_amend_order_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdAmendOrderPostError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_books_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdBooksGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_cancel_all_after_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdCancelAllAfterGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_cancel_order_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdCancelOrderPostError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_get_trades_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdGetTradesGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_mass_cancel_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdMassCancelPostError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_order_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdOrderGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_order_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdOrderPostError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_orders_history_archive_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdOrdersHistoryArchiveGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_orders_history_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdOrdersHistoryGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_orders_pending_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdOrdersPendingGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_public_ticker_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdPublicTickerGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sprd_spreads_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SprdSpreadsGetError {
    Status400(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Retrieve the candlestick charts. This endpoint can retrieve the latest 1,440 data entries. Charts are returned in groups based on the requested bar.<br>Rate Limit: 40 requests per 2 seconds<br>Rate limit rule: IP.<br>Rate limit rule: UserID
pub async fn market_sprd_candles_get(configuration: &configuration::Configuration, sprd_id: &str, bar: Option<&str>, after: Option<String>, before: Option<String>, limit: Option<i32>) -> Result<models::MarketSprdCandlesGet200Response, Error<MarketSprdCandlesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_sprd_id = sprd_id;
    let p_bar = bar;
    let p_after = after;
    let p_before = before;
    let p_limit = limit;

    let uri_str = format!("{}/market/sprd-candles", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("sprdId", &p_sprd_id.to_string())]);
    if let Some(ref param_value) = p_bar {
        req_builder = req_builder.query(&[("bar", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_before {
        req_builder = req_builder.query(&[("before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MarketSprdCandlesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve history candlestick charts from recent years.<br>Rate limit rule: IP.<br>Rate limit rule: UserID
pub async fn market_sprd_history_candles_get(configuration: &configuration::Configuration, sprd_id: &str, bar: Option<&str>, after: Option<String>, before: Option<String>, limit: Option<i32>) -> Result<models::MarketSprdCandlesGet200Response, Error<MarketSprdHistoryCandlesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_sprd_id = sprd_id;
    let p_bar = bar;
    let p_after = after;
    let p_before = before;
    let p_limit = limit;

    let uri_str = format!("{}/market/sprd-history-candles", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("sprdId", &p_sprd_id.to_string())]);
    if let Some(ref param_value) = p_bar {
        req_builder = req_builder.query(&[("bar", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_after {
        req_builder = req_builder.query(&[("after", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_before {
        req_builder = req_builder.query(&[("before", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MarketSprdHistoryCandlesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn market_sprd_ticker_get(configuration: &configuration::Configuration, sprd_id: Option<&str>) -> Result<models::MarketTickersGet200Response, Error<MarketSprdTickerGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_sprd_id = sprd_id;

    let uri_str = format!("{}/market/sprd-ticker", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_sprd_id {
        req_builder = req_builder.query(&[("sprdId", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<MarketSprdTickerGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Amend an incomplete order.<br>Rate Limit: 20 requests per 2 seconds <br>Rate limit rule: UserID
pub async fn sprd_amend_order_post(configuration: &configuration::Configuration, amend_order: Option<models::AmendOrder>) -> Result<models::TradeOrderPost200Response, Error<SprdAmendOrderPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_amend_order = amend_order;

    let uri_str = format!("{}/sprd/amend-order", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_amend_order);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdAmendOrderPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve all order books.<br>Rate limit rule: UserID
pub async fn sprd_books_get(configuration: &configuration::Configuration, sprd_id: Option<&str>, sz: Option<&str>) -> Result<models::MarketBooksGet200Response, Error<SprdBooksGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_sprd_id = sprd_id;
    let p_sz = sz;

    let uri_str = format!("{}/sprd/books", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_sprd_id {
        req_builder = req_builder.query(&[("sprdId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sz {
        req_builder = req_builder.query(&[("sz", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdBooksGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Cancel all pending orders after the countdown timeout. Only applicable to spread trading.<br>Rate limit rule: IP.<br>Rate limit rule: UserID
pub async fn sprd_cancel_all_after_get(configuration: &configuration::Configuration, time_out: &str) -> Result<models::SprdCancelAllAfterGet200Response, Error<SprdCancelAllAfterGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_time_out = time_out;

    let uri_str = format!("{}/sprd/cancel-all-after", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("timeOut", &p_time_out.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdCancelAllAfterGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// cancel order<br>Rate Limit: 20 requests per 2 seconds <br>Rate limit rule: UserID
pub async fn sprd_cancel_order_post(configuration: &configuration::Configuration, cancel_order_request: Option<models::CancelOrderRequest>) -> Result<models::TradeOrderPost200Response, Error<SprdCancelOrderPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_cancel_order_request = cancel_order_request;

    let uri_str = format!("{}/sprd/cancel-order", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_cancel_order_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdCancelOrderPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve historical transaction details for the last 7 days. Results are returned in counter chronological order.<br>Rate limit rule: UserID
pub async fn sprd_get_trades_get(configuration: &configuration::Configuration, sprd_id: Option<&str>, trade_id: Option<&str>, ord_id: Option<&str>, begin_id: Option<&str>, end_id: Option<&str>, begin: Option<&str>, end: Option<&str>, limit: Option<i32>) -> Result<models::SprdGetTradesGet200Response, Error<SprdGetTradesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_sprd_id = sprd_id;
    let p_trade_id = trade_id;
    let p_ord_id = ord_id;
    let p_begin_id = begin_id;
    let p_end_id = end_id;
    let p_begin = begin;
    let p_end = end;
    let p_limit = limit;

    let uri_str = format!("{}/sprd/get-trades", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_sprd_id {
        req_builder = req_builder.query(&[("sprdId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_trade_id {
        req_builder = req_builder.query(&[("tradeId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_ord_id {
        req_builder = req_builder.query(&[("ordId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_begin_id {
        req_builder = req_builder.query(&[("beginId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_id {
        req_builder = req_builder.query(&[("endId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_begin {
        req_builder = req_builder.query(&[("begin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end {
        req_builder = req_builder.query(&[("end", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdGetTradesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// cancel all order<br>Rate Limit: 20 requests per 2 seconds <br>Rate limit rule: UserID
pub async fn sprd_mass_cancel_post(configuration: &configuration::Configuration, sprd_mass_cancel_post_request: Option<models::SprdMassCancelPostRequest>) -> Result<models::CopytradingFirstCopySettingPost200Response, Error<SprdMassCancelPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_sprd_mass_cancel_post_request = sprd_mass_cancel_post_request;

    let uri_str = format!("{}/sprd/mass-cancel", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_sprd_mass_cancel_post_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdMassCancelPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// get order<br>Rate Limit: 20 requests per 2 seconds <br>Rate limit rule: UserID
pub async fn sprd_order_get(configuration: &configuration::Configuration, ord_id: &str, cl_ord_id: &str) -> Result<models::SprdOrderGet200Response, Error<SprdOrderGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_ord_id = ord_id;
    let p_cl_ord_id = cl_ord_id;

    let uri_str = format!("{}/sprd/order", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("ordId", &p_ord_id.to_string())]);
    req_builder = req_builder.query(&[("clOrdId", &p_cl_ord_id.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdOrderGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Place a new order<br>Rate Limit: 20 requests per 2 seconds <br>Rate limit rule: UserID
pub async fn sprd_order_post(configuration: &configuration::Configuration, spread_order: Option<models::SpreadOrder>) -> Result<models::TradeOrderPost200Response, Error<SprdOrderPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_spread_order = spread_order;

    let uri_str = format!("{}/sprd/order", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_spread_order);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdOrderPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve the completed order data for the last 3 months, including those placed 3 months ago but completed in the last 3 months. Results are returned in counter chronological order.<br>Rate limit rule: UserID
pub async fn sprd_orders_history_archive_get(configuration: &configuration::Configuration, sprd_id: Option<&str>, ord_type: Option<&str>, state: Option<&str>, begin_id: Option<&str>, end_id: Option<&str>, begin: Option<&str>, end: Option<&str>, limit: Option<i32>) -> Result<models::SprdOrderGet200Response, Error<SprdOrdersHistoryArchiveGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_sprd_id = sprd_id;
    let p_ord_type = ord_type;
    let p_state = state;
    let p_begin_id = begin_id;
    let p_end_id = end_id;
    let p_begin = begin;
    let p_end = end;
    let p_limit = limit;

    let uri_str = format!("{}/sprd/orders-history-archive", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_sprd_id {
        req_builder = req_builder.query(&[("sprdId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_ord_type {
        req_builder = req_builder.query(&[("ordType", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_state {
        req_builder = req_builder.query(&[("state", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_begin_id {
        req_builder = req_builder.query(&[("beginId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_id {
        req_builder = req_builder.query(&[("endId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_begin {
        req_builder = req_builder.query(&[("begin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end {
        req_builder = req_builder.query(&[("end", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdOrdersHistoryArchiveGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Amend an incomplete order.<br>Rate Limit: 20 requests per 2 seconds <br>Rate limit rule: UserID
pub async fn sprd_orders_history_get(configuration: &configuration::Configuration, sprd_id: Option<&str>, ord_type: Option<&str>, state: Option<&str>, begin_id: Option<&str>, end_id: Option<&str>, begin: Option<&str>, end: Option<&str>, limit: Option<i32>) -> Result<models::SprdOrderGet200Response, Error<SprdOrdersHistoryGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_sprd_id = sprd_id;
    let p_ord_type = ord_type;
    let p_state = state;
    let p_begin_id = begin_id;
    let p_end_id = end_id;
    let p_begin = begin;
    let p_end = end;
    let p_limit = limit;

    let uri_str = format!("{}/sprd/orders-history", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_sprd_id {
        req_builder = req_builder.query(&[("sprdId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_ord_type {
        req_builder = req_builder.query(&[("ordType", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_state {
        req_builder = req_builder.query(&[("state", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_begin_id {
        req_builder = req_builder.query(&[("beginId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_id {
        req_builder = req_builder.query(&[("endId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_begin {
        req_builder = req_builder.query(&[("begin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end {
        req_builder = req_builder.query(&[("end", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdOrdersHistoryGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Amend an incomplete order.<br>Rate Limit: 20 requests per 2 seconds <br>Rate limit rule: UserID
pub async fn sprd_orders_pending_get(configuration: &configuration::Configuration, sprd_id: Option<&str>, ord_type: Option<&str>, state: Option<&str>, begin_id: Option<&str>, end_id: Option<&str>, limit: Option<i32>) -> Result<models::SprdOrderGet200Response, Error<SprdOrdersPendingGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_sprd_id = sprd_id;
    let p_ord_type = ord_type;
    let p_state = state;
    let p_begin_id = begin_id;
    let p_end_id = end_id;
    let p_limit = limit;

    let uri_str = format!("{}/sprd/orders-pending", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_sprd_id {
        req_builder = req_builder.query(&[("sprdId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_ord_type {
        req_builder = req_builder.query(&[("ordType", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_state {
        req_builder = req_builder.query(&[("state", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_begin_id {
        req_builder = req_builder.query(&[("beginId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_id {
        req_builder = req_builder.query(&[("endId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdOrdersPendingGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve all order books.<br>Rate limit rule: UserID
pub async fn sprd_public_ticker_get(configuration: &configuration::Configuration, sprd_id: Option<&str>) -> Result<models::SprdPublicTickerGet200Response, Error<SprdPublicTickerGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_sprd_id = sprd_id;

    let uri_str = format!("{}/sprd/public-ticker", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_sprd_id {
        req_builder = req_builder.query(&[("sprdId", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdPublicTickerGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Retrieve all available spreads based on the request parameters.<br>Rate limit rule: UserID
pub async fn sprd_spreads_get(configuration: &configuration::Configuration, base_ccy: Option<&str>, inst_id: Option<&str>, sprd_id: Option<&str>, state: Option<&str>) -> Result<models::SprdGetTradesGet200Response, Error<SprdSpreadsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_base_ccy = base_ccy;
    let p_inst_id = inst_id;
    let p_sprd_id = sprd_id;
    let p_state = state;

    let uri_str = format!("{}/sprd/spreads", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_base_ccy {
        req_builder = req_builder.query(&[("baseCcy", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_inst_id {
        req_builder = req_builder.query(&[("instId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sprd_id {
        req_builder = req_builder.query(&[("sprdId", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_state {
        req_builder = req_builder.query(&[("state", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<SprdSpreadsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

