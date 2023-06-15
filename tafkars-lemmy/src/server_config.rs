use actix_web::ResponseError;

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Clone)]
pub struct GatewayConfig {
    pub lemmy_url: String,
}

#[derive(Error, Debug)]
pub enum ServerSideError {
    #[error("API request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("failed to parse int: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Misconfigured gateway")]
    MisconfigurationError,
}

impl ResponseError for ServerSideError {}
