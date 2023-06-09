use actix_web::http::header::ContentType;
use actix_web::{get, web, HttpRequest, HttpResponse};
use lemmy_api_common::comment::GetComments;
use lemmy_api_common::lemmy_db_schema::newtypes::PostId;
use lemmy_api_common::lemmy_db_schema::{CommentSortType, ListingType};
use lemmy_api_common::post::{GetPost, GetPosts};
use tafkars::listing::Listing;

use crate::api_translation;
use crate::web_config;

#[get("/")]
async fn web_root() -> Result<HttpResponse, web_config::MyError> {
    let message = "Thank you for using tafkars! To see more info and documentation, please see the repo: https://github.com/derivator/tafkars";
    Ok(HttpResponse::Ok()
        .insert_header(ContentType::json())
        .body(serde_json::to_string(&message)?))
}

#[get("/r/{subreddit}/{sorting}{_:/?}.json")]
async fn subreddit(
    req: HttpRequest,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, web_config::MyError> {
    let state = web_config::prepare(&req)?;
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
async fn frontpage(
    req: HttpRequest,
    path: web::Path<(String,)>,
) -> Result<HttpResponse, web_config::MyError> {
    let state = web_config::prepare(&req)?;
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
) -> Result<HttpResponse, web_config::MyError> {
    let state = web_config::prepare(&req)?;
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
