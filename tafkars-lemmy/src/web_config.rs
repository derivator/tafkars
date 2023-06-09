use actix_web::{HttpRequest, ResponseError};
use lemmy_api_common::comment::{GetComments, GetCommentsResponse};
use lemmy_api_common::lemmy_db_schema::newtypes::DbUrl;

use lemmy_api_common::post::{GetPost, GetPostResponse, GetPosts, GetPostsResponse};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Clone)]
pub struct GatewayConfig {
    pub lemmy_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
}

pub struct ResponseConfig {
    /// HTML-escape body_html in responses?
    pub raw_json: bool,
    /// Escape names like user@instance.xyz to user__instance_xyz in API responses?
    pub escape_names: bool,
    /// Unescape names given in API requests?
    pub unescape_names: bool,
}

pub struct ResponseState<'a> {
    pub app: &'a AppState,
    pub config: &'a GatewayConfig,
    pub res_config: ResponseConfig,
}

#[derive(Error, Debug)]
pub enum MyError {
    #[error("API request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("failed to parse int: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Misconfigured gateway")]
    MisconfigurationError,
}

impl ResponseError for MyError {}

pub fn prepare(req: &HttpRequest) -> Result<ResponseState, MyError> {
    let app = &req
        .app_data::<AppState>()
        .ok_or(MyError::MisconfigurationError)?;

    let config = &req
        .app_data::<GatewayConfig>()
        .ok_or(MyError::MisconfigurationError)?;

    let _user_agent = req.headers().get("user-agent");

    let res_config = ResponseConfig {
        raw_json: req.query_string().contains("raw_json=1"),
        escape_names: true,
        unescape_names: true,
    };

    let state = ResponseState {
        app,
        config,
        res_config,
    };
    // TODO: use headers to determine how much deviation from standard API this client can handle
    Ok(state)
}

impl<'a> ResponseState<'a> {
    pub fn escape_actor_id_str(&self, actor_id: &str) -> Option<String> {
        if let [instance, _ty, name] = actor_id
            .split("://")
            .last()?
            .split('/')
            .collect::<Vec<&str>>()[..]
        {
            if self.res_config.escape_names {
                let instance = instance.replace('.', "_");
                Some(format!("{name}__{instance}"))
            } else {
                Some(format!("{name}@{instance}"))
            }
        } else {
            None
        }
    }

    pub fn escape_actor_id(&self, actor_id: &DbUrl) -> Option<String> {
        self.escape_actor_id_str(actor_id.as_str())
    }

    pub fn unescape_name(&self, escaped: &str) -> Option<String> {
        if self.res_config.unescape_names {
            let (name, instance) = escaped.rsplit_once("__")?;
            Some(format!("{name}@{instance}"))
        } else {
            Some(escaped.to_owned())
        }
    }

    pub async fn api_call(
        &self,
        endpoint: &str,
        params: &impl Serialize,
    ) -> Result<String, MyError> {
        let api_url = &self.config.lemmy_url;
        Ok(self
            .app
            .http_client
            .get(format!("{api_url}/{endpoint}"))
            .query(params)
            .send()
            .await?
            .text()
            .await?)
    }

    pub async fn api_call_typed<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &impl Serialize,
    ) -> Result<T, MyError> {
        let res = self.api_call(endpoint, params).await?;
        Ok(serde_json::from_str(&res)?)
    }

    pub async fn get_post(&self, params: &GetPost) -> Result<GetPostResponse, MyError> {
        self.api_call_typed("api/v3/post", params).await
    }

    pub async fn get_posts(&self, params: &GetPosts) -> Result<GetPostsResponse, MyError> {
        self.api_call_typed("api/v3/post/list", params).await
    }

    pub async fn get_comments(&self, params: &GetComments) -> Result<GetCommentsResponse, MyError> {
        self.api_call_typed("api/v3/comment/list", params).await
    }
}
