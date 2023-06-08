use actix_web::http::header::ContentType;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, ResponseError};
use lemmy_api_common::comment::{GetComments, GetCommentsResponse};
use lemmy_api_common::lemmy_db_schema::newtypes::PostId;
use lemmy_api_common::lemmy_db_schema::{CommentSortType, ListingType};
use lemmy_api_common::post::{GetPost, GetPostResponse, GetPosts, GetPostsResponse};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;
use std::num::ParseIntError;

pub mod api_translation;

#[derive(Clone)]
pub struct GatewayConfig {
    lemmy_url: String,
}

#[derive(Clone)]
pub struct AppState {
    http_client: Client,
}

pub struct ResponseConfig {
    /// HTML-escape body_html in responses?
    raw_json: bool,
    /// Escape names like user@instance.xyz to user__instance_xyz in API responses?
    escape_names: bool,
    /// Unescape names given in API requests?
    unescape_names: bool,
}

pub struct ResponseState<'a> {
    app: &'a AppState,
    config: &'a GatewayConfig,
    res_config: ResponseConfig,
}

impl<'a> ResponseState<'a> {
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

use tafkars::listing::Listing;
use thiserror::Error;

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

pub fn prepare(req: &HttpRequest) -> Result<ResponseState, MyError> {
    let app = &req
        .app_data::<AppState>()
        .ok_or(MyError::MisconfigurationError)?;

    let config = &req
        .app_data::<GatewayConfig>()
        .ok_or(MyError::MisconfigurationError)?;

    let user_agent = req.headers().get("user-agent");

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

impl ResponseError for MyError {}

#[get("/r/{subreddit}/{sorting}{_:/?}.json")]
async fn subreddit(
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, MyError> {
    let state = prepare(&req)?;
    let (subreddit, _sorting) = path.into_inner(); // TODO: apply sorting

    let subreddit = state.unescape_name(&subreddit).unwrap_or(subreddit);

    let params = GetPosts {
        sort: None,
        community_name: Some(subreddit.to_string()),
        auth: None,
        ..Default::default()
    };

    let res = state.get_posts(&params).await?;
    let posts = api_translation::posts(&state, res);
    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .body(serde_json::to_string(&posts)?))
}

#[get("/{sorting}{_:/?}.json")]
async fn frontpage(req: HttpRequest, path: web::Path<(String,)>) -> Result<HttpResponse, MyError> {
    let state = prepare(&req)?;
    let (_sorting,) = path.into_inner(); // TODO: apply sorting
    let params = GetPosts {
        sort: None,
        auth: None,
        type_: Some(ListingType::All),
        ..Default::default()
    };

    let res = state.get_posts(&params).await?;
    let posts = api_translation::posts(&state, res);
    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .body(serde_json::to_string(&posts)?))
}

#[get("/comments/{post_id}{_:/?}.json")]
async fn comments_for_post(
    req: HttpRequest,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, MyError> {
    let state = prepare(&req)?;
    let post_id = path.into_inner().0.parse()?;

    let res = state
        .get_post(&GetPost {
            id: Some(PostId(post_id)),
            auth: None,
            ..Default::default()
        })
        .await?;
    let post = api_translation::post(&state, res.post_view);
    let post_listing = Listing::new(vec![post]);
    let res = state
        .get_comments(&GetComments {
            type_: Some(ListingType::All),
            sort: Some(CommentSortType::Hot),
            max_depth: None,
            page: None,
            limit: Some(100),
            post_id: Some(PostId(post_id)),
            auth: None,
            ..Default::default()
        })
        .await?;
    let comments = api_translation::comments(&state, res);

    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .body(serde_json::to_string(&(post_listing, comments))?))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // test with RUST_LOG=info to see requests

    let lemmy_url: String = env::args()
        .nth(1)
        .expect("please providy a lemmy instance url as a cmd arg");
    let config = GatewayConfig { lemmy_url };

    let app_state = AppState {
        http_client: Default::default(),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(frontpage)
            .service(subreddit)
            .service(comments_for_post)
            .app_data(app_state.clone())
            .app_data(config.clone())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
