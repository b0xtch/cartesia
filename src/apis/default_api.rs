/*
 * Cartesia REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2024-06-10
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use reqwest;
use serde::{Deserialize, Serialize};

/// struct for passing parameters to the method [`clone_voice_from_clip`]
#[derive(Clone, Debug)]
pub struct CloneVoiceFromClipParams {
    /// The version of the Cartesia API to use.
    pub cartesia_version: String,
    pub clip: Option<std::path::PathBuf>,
}

/// struct for passing parameters to the method [`tts_bytes_post`]
#[derive(Clone, Debug)]
pub struct TtsBytesPostParams {
    /// The version of the Cartesia API to use.
    pub cartesia_version: String,
    pub tts_request: models::TtsRequest,
}

/// struct for passing parameters to the method [`tts_sse_post`]
#[derive(Clone, Debug)]
pub struct TtsSsePostParams {
    /// The version of the Cartesia API to use.
    pub cartesia_version: String,
    pub tts_request: models::TtsRequest,
}

/// struct for passing parameters to the method [`voices_clone_url_post`]
#[derive(Clone, Debug)]
pub struct VoicesCloneUrlPostParams {
    /// The version of the Cartesia API to use.
    pub cartesia_version: String,
    pub link: String,
}

/// struct for passing parameters to the method [`voices_get`]
#[derive(Clone, Debug)]
pub struct VoicesGetParams {
    /// The version of the Cartesia API to use.
    pub cartesia_version: String,
}

/// struct for passing parameters to the method [`voices_id_delete`]
#[derive(Clone, Debug)]
pub struct VoicesIdDeleteParams {
    /// The version of the Cartesia API to use.
    pub cartesia_version: String,
    pub id: String,
}

/// struct for passing parameters to the method [`voices_id_get`]
#[derive(Clone, Debug)]
pub struct VoicesIdGetParams {
    /// The version of the Cartesia API to use.
    pub cartesia_version: String,
    pub id: String,
}

/// struct for passing parameters to the method [`voices_post`]
#[derive(Clone, Debug)]
pub struct VoicesPostParams {
    /// The version of the Cartesia API to use.
    pub cartesia_version: String,
    pub voices_post_request: models::VoicesPostRequest,
}

/// struct for typed errors of method [`clone_voice_from_clip`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CloneVoiceFromClipError {
    DefaultResponse(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`root_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RootGetError {
    DefaultResponse(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`tts_bytes_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TtsBytesPostError {
    DefaultResponse(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`tts_sse_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TtsSsePostError {
    DefaultResponse(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`voices_clone_url_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoicesCloneUrlPostError {
    DefaultResponse(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`voices_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoicesGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`voices_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoicesIdDeleteError {
    DefaultResponse(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`voices_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoicesIdGetError {
    DefaultResponse(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`voices_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoicesPostError {
    DefaultResponse(String),
    UnknownValue(serde_json::Value),
}

/// Clones a voice from an audio clip uploaded as a file. The clip is uploaded using multipart/form-data with a `clip` field containing the audio file.
pub async fn clone_voice_from_clip(
    configuration: &configuration::Configuration,
    params: CloneVoiceFromClipParams,
) -> Result<models::VoiceEmbedding, Error<CloneVoiceFromClipError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let cartesia_version = params.cartesia_version;
    let clip = params.clip;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/voices/clone/clip", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("Cartesia-Version", cartesia_version.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Key", local_var_value);
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'clip' parameter
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CloneVoiceFromClipError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the server's version and a status message which can be useful for sanity checking.
pub async fn root_get(
    configuration: &configuration::Configuration,
) -> Result<models::Get200Response, Error<RootGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RootGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Generate audio from a transcript using a given voice and model. The audio is streamed out as raw bytes.
pub async fn tts_bytes_post(
    configuration: &configuration::Configuration,
    params: TtsBytesPostParams,
) -> Result<std::path::PathBuf, Error<TtsBytesPostError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let cartesia_version = params.cartesia_version;
    let tts_request = params.tts_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tts/bytes", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("Cartesia-Version", cartesia_version.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&tts_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TtsBytesPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Generate audio from a transcript using a given voice and model. The audio is streamed out as Server-Sent Events.
pub async fn tts_sse_post(
    configuration: &configuration::Configuration,
    params: TtsSsePostParams,
) -> Result<models::TtsSsePost200Response, Error<TtsSsePostError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let cartesia_version = params.cartesia_version;
    let tts_request = params.tts_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/tts/sse", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("Cartesia-Version", cartesia_version.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&tts_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TtsSsePostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Clone a voice from an online URL. Currently only supports YouTube.
pub async fn voices_clone_url_post(
    configuration: &configuration::Configuration,
    params: VoicesCloneUrlPostParams,
) -> Result<models::VoiceEmbedding, Error<VoicesCloneUrlPostError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let cartesia_version = params.cartesia_version;
    let link = params.link;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/voices/clone/url", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("link", &link.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("Cartesia-Version", cartesia_version.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<VoicesCloneUrlPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all voices available to the user, that is, public voices and the user's own voices.
pub async fn voices_get(
    configuration: &configuration::Configuration,
    params: VoicesGetParams,
) -> Result<Vec<models::Voice>, Error<VoicesGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let cartesia_version = params.cartesia_version;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/voices", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("Cartesia-Version", cartesia_version.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<VoicesGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn voices_id_delete(
    configuration: &configuration::Configuration,
    params: VoicesIdDeleteParams,
) -> Result<(), Error<VoicesIdDeleteError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let cartesia_version = params.cartesia_version;
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/voices/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("Cartesia-Version", cartesia_version.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<VoicesIdDeleteError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn voices_id_get(
    configuration: &configuration::Configuration,
    params: VoicesIdGetParams,
) -> Result<models::Voice, Error<VoicesIdGetError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let cartesia_version = params.cartesia_version;
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/voices/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("Cartesia-Version", cartesia_version.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<VoicesIdGetError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new voice with a given name, description, and embedding.
pub async fn voices_post(
    configuration: &configuration::Configuration,
    params: VoicesPostParams,
) -> Result<models::Voice, Error<VoicesPostError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let cartesia_version = params.cartesia_version;
    let voices_post_request = params.voices_post_request;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/voices", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder =
        local_var_req_builder.header("Cartesia-Version", cartesia_version.to_string());
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("X-API-Key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&voices_post_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<VoicesPostError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
