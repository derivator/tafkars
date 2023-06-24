use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpRequest, HttpResponse};
use lemmy_api_common::comment::{GetComments, GetCommentsResponse};
use lemmy_api_common::community::{GetCommunity, GetCommunityResponse};
use lemmy_api_common::lemmy_db_schema::newtypes::{DbUrl, PostId};
use lemmy_api_common::lemmy_db_schema::ListingType;
use lemmy_api_common::post::{GetPost, GetPostResponse, GetPosts, GetPostsResponse};
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use tafkars::listing::Listing;
use tafkars::{submission, subreddit};

use crate::api_translation;
use crate::server_config;

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

impl ResponseConfig {
    pub fn markdown_to_html(&self, md_text: &str) -> String {
        let html = markdown::to_html(md_text);
        if self.raw_json {
            html
        } else {
            html_escape::encode_safe(&html).to_string()
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web_root)
        .service(frontpage)
        .service(community_about)
        .service(
            web::resource([
                "/r/{subreddit}{_:/?}.json",
                "/r/{subreddit}/{sorting}{_:/?}.json",
            ])
            .route(web::get().to(community)),
        )
        .service(comments_for_post);
}

pub struct ResponseState<'a> {
    pub app: &'a AppState,
    pub config: &'a server_config::GatewayConfig,
    pub res_config: ResponseConfig,
}

pub fn prepare(req: &HttpRequest) -> Result<ResponseState, server_config::ServerSideError> {
    let app = &req
        .app_data::<AppState>()
        .ok_or(server_config::ServerSideError::MisconfigurationError)?;

    let config = &req
        .app_data::<server_config::GatewayConfig>()
        .ok_or(server_config::ServerSideError::MisconfigurationError)?;

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
    ) -> Result<String, server_config::ServerSideError> {
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
    ) -> Result<T, server_config::ServerSideError> {
        let res = self.api_call(endpoint, params).await?;
        Ok(serde_json::from_str(&res)?)
    }

    pub async fn get_post(
        &self,
        params: &GetPost,
    ) -> Result<GetPostResponse, server_config::ServerSideError> {
        self.api_call_typed("api/v3/post", params).await
    }

    pub async fn get_posts(
        &self,
        params: &GetPosts,
    ) -> Result<GetPostsResponse, server_config::ServerSideError> {
        self.api_call_typed("api/v3/post/list", params).await
    }

    pub async fn get_comments(
        &self,
        params: &GetComments,
    ) -> Result<GetCommentsResponse, server_config::ServerSideError> {
        self.api_call_typed("api/v3/comment/list", params).await
    }

    pub async fn get_community(
        &self,
        params: &GetCommunity,
    ) -> Result<GetCommunityResponse, server_config::ServerSideError> {
        self.api_call_typed("api/v3/community", params).await
    }
}

pub fn respond_json<T: Serialize>(
    data: &T,
) -> Result<HttpResponse, server_config::ServerSideError> {
    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .body(serde_json::to_string(data)?))
}

#[get("/")]
async fn web_root() -> Result<HttpResponse, server_config::ServerSideError> {
    let message = "Thank you for using tafkars! To see more info and documentation, please see the repo: https://github.com/derivator/tafkars";
    respond_json(&message)
}

#[get("/r/{subreddit}/about{_:/?}.json")]
async fn community_about(
    req: HttpRequest,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, server_config::ServerSideError> {
    let state = prepare(&req)?;
    let (sr,) = path.into_inner();

    let sr = state.unescape_name(&sr).unwrap_or(sr);

    let params = GetCommunity {
        name: Some(sr),
        auth: None,
        ..Default::default()
    };

    let res = state.get_community(&params).await?;
    let com = api_translation::community(&state, res.community_view);
    respond_json(&com)
}

#[derive(Debug, Deserialize)]
struct CommunityPath {
    subreddit: String,
    sorting: Option<subreddit::SortOrder>,
}

async fn community(
    req: HttpRequest,
    path: web::Path<CommunityPath>,
    query: web::Query<subreddit::Query>,
) -> Result<HttpResponse, server_config::ServerSideError> {
    let state = prepare(&req)?;
    let path = path.into_inner();

    let sort = path
        .sorting
        .and_then(|s| api_translation::submission_sort(s, query.0.time));

    let subreddit = path.subreddit;
    let subreddit = state.unescape_name(&subreddit).unwrap_or(subreddit);

    let params = GetPosts {
        sort,
        community_name: Some(subreddit),
        auth: None,
        ..Default::default()
    };

    let res = state.get_posts(&params).await?;
    let posts = api_translation::posts(&state, res);
    respond_json(&posts)
}

#[get("/{sorting}{_:/?}.json")]
async fn frontpage(
    req: HttpRequest,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, server_config::ServerSideError> {
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
    respond_json(&posts)
}

#[get("/comments/{post_id}{_:/?}.json")]
async fn comments_for_post(
    req: HttpRequest,
    path: web::Path<(String,)>,
    query: web::Query<submission::Query>,
) -> Result<HttpResponse, server_config::ServerSideError> {
    let state = prepare(&req)?;
    let post_id = path.into_inner().0.parse()?;
    let query = query.0;
    let sort = query.sort.map(api_translation::comment_sort).flatten();

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
            sort,
            max_depth: query.depth.map(|x| x as i32),
            page: None,
            limit: Some(query.limit.unwrap_or(100) as i64),
            post_id: Some(PostId(post_id)),
            auth: None,
            ..Default::default()
        })
        .await?;
    let comments = api_translation::comments(&state, res);

    respond_json(&(post_listing, comments))
}
