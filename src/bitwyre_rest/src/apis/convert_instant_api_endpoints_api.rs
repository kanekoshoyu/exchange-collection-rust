/*
 * Bitwyre REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`private_convert_callback_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateConvertCallbackPutError {
    Status500(),
    Status400(),
    Status401(),
    Status404(),
    Status405(),
    Status406(),
    Status410(),
    Status418(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`private_convert_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateConvertGetError {
    Status500(),
    Status400(),
    Status401(),
    Status404(),
    Status405(),
    Status406(),
    Status410(),
    Status418(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`private_convert_payment_methods_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateConvertPaymentMethodsGetError {
    Status500(),
    Status400(),
    Status401(),
    Status404(),
    Status405(),
    Status406(),
    Status410(),
    Status418(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`private_convert_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateConvertPostError {
    Status500(),
    Status400(),
    Status401(),
    Status404(),
    Status405(),
    Status406(),
    Status410(),
    Status418(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`private_convert_status_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateConvertStatusGetError {
    Status500(),
    Status400(),
    Status401(),
    Status404(),
    Status405(),
    Status406(),
    Status410(),
    Status418(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`private_convert_vault_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateConvertVaultGetError {
    Status500(),
    Status400(),
    Status401(),
    Status404(),
    Status405(),
    Status406(),
    Status410(),
    Status418(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`private_convert_vault_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateConvertVaultPostError {
    Status500(),
    Status400(),
    Status401(),
    Status404(),
    Status405(),
    Status406(),
    Status410(),
    Status418(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`private_deposit_callback_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateDepositCallbackPutError {
    Status500(),
    Status400(),
    Status401(),
    Status404(),
    Status405(),
    Status406(),
    Status410(),
    Status418(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`private_withdrawal_callback_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateWithdrawalCallbackPutError {
    Status500(),
    Status400(),
    Status401(),
    Status404(),
    Status405(),
    Status406(),
    Status410(),
    Status418(),
    Status422(),
    Status429(),
    Status503(),
    UnknownValue(serde_json::Value),
}


pub async fn private_convert_callback_put(configuration: &configuration::Configuration, request_schema: models::RequestSchema) -> Result<models::PrivateDepositCallbackPut200Response, Error<PrivateConvertCallbackPutError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_request_schema = request_schema;

    let uri_str = format!("{}/private/convert/callback", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };
    req_builder = req_builder.json(&p_request_schema);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateConvertCallbackPutError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn private_convert_get(configuration: &configuration::Configuration, nonce: i32, checksum: &str, convert_uuid: &str) -> Result<models::PrivateConvertGet200Response, Error<PrivateConvertGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_nonce = nonce;
    let p_checksum = checksum;
    let p_convert_uuid = convert_uuid;

    let uri_str = format!("{}/private/convert", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("nonce", &p_nonce.to_string())]);
    req_builder = req_builder.query(&[("checksum", &p_checksum.to_string())]);
    req_builder = req_builder.query(&[("convert_uuid", &p_convert_uuid.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateConvertGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn private_convert_payment_methods_get(configuration: &configuration::Configuration, nonce: i32, checksum: &str, r#type: &str) -> Result<models::PrivateConvertPaymentMethodsGet200Response, Error<PrivateConvertPaymentMethodsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_nonce = nonce;
    let p_checksum = checksum;
    let p_type = r#type;

    let uri_str = format!("{}/private/convert/payment_methods", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("nonce", &p_nonce.to_string())]);
    req_builder = req_builder.query(&[("checksum", &p_checksum.to_string())]);
    req_builder = req_builder.query(&[("type", &p_type.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateConvertPaymentMethodsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn private_convert_post(configuration: &configuration::Configuration, request_schema: models::RequestSchema) -> Result<models::PrivateConvertPost200Response, Error<PrivateConvertPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_request_schema = request_schema;

    let uri_str = format!("{}/private/convert", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };
    req_builder = req_builder.json(&p_request_schema);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateConvertPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn private_convert_status_get(configuration: &configuration::Configuration, nonce: i32, checksum: &str, convert_uuid: &str) -> Result<models::PrivateConvertStatusGet200Response, Error<PrivateConvertStatusGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_nonce = nonce;
    let p_checksum = checksum;
    let p_convert_uuid = convert_uuid;

    let uri_str = format!("{}/private/convert/status", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("nonce", &p_nonce.to_string())]);
    req_builder = req_builder.query(&[("checksum", &p_checksum.to_string())]);
    req_builder = req_builder.query(&[("convert_uuid", &p_convert_uuid.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateConvertStatusGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn private_convert_vault_get(configuration: &configuration::Configuration, nonce: i32, checksum: &str, asset: &str, network_id: Option<&str>, convert_uuid: Option<&str>, address: Option<&str>, tx_id: Option<&str>, page: Option<i32>, per_page: Option<i32>) -> Result<models::PrivateConvertVaultGet200Response, Error<PrivateConvertVaultGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_nonce = nonce;
    let p_checksum = checksum;
    let p_asset = asset;
    let p_network_id = network_id;
    let p_convert_uuid = convert_uuid;
    let p_address = address;
    let p_tx_id = tx_id;
    let p_page = page;
    let p_per_page = per_page;

    let uri_str = format!("{}/private/convert/vault", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("nonce", &p_nonce.to_string())]);
    req_builder = req_builder.query(&[("checksum", &p_checksum.to_string())]);
    req_builder = req_builder.query(&[("asset", &p_asset.to_string())]);
    if let Some(ref param_value) = p_network_id {
        req_builder = req_builder.query(&[("network_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_convert_uuid {
        req_builder = req_builder.query(&[("convert_uuid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_address {
        req_builder = req_builder.query(&[("address", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_tx_id {
        req_builder = req_builder.query(&[("tx_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_per_page {
        req_builder = req_builder.query(&[("per_page", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateConvertVaultGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn private_convert_vault_post(configuration: &configuration::Configuration, request_schema: models::RequestSchema) -> Result<models::PrivateConvertVaultPost200Response, Error<PrivateConvertVaultPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_request_schema = request_schema;

    let uri_str = format!("{}/private/convert/vault", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };
    req_builder = req_builder.json(&p_request_schema);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateConvertVaultPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn private_deposit_callback_put(configuration: &configuration::Configuration, request_schema: models::RequestSchema) -> Result<models::PrivateDepositCallbackPut200Response, Error<PrivateDepositCallbackPutError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_request_schema = request_schema;

    let uri_str = format!("{}/private/deposit/callback", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };
    req_builder = req_builder.json(&p_request_schema);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateDepositCallbackPutError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn private_withdrawal_callback_put(configuration: &configuration::Configuration, request_schema: models::RequestSchema) -> Result<models::PrivateDepositCallbackPut200Response, Error<PrivateWithdrawalCallbackPutError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_request_schema = request_schema;

    let uri_str = format!("{}/private/withdrawal/callback", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };
    req_builder = req_builder.json(&p_request_schema);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateWithdrawalCallbackPutError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

