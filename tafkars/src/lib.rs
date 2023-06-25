//! # The API formerly known as...
//! Forked from [`roux`](https://docs.rs/roux/) to get just the API definitions and simplify a bit
pub mod comment;
pub mod listing;
pub mod submission;
pub mod subreddit;
pub mod user;

use crate::comment::CommentData;
use crate::listing::ListingData;
use crate::submission::SubmissionData;
use crate::subreddit::SubredditData;
use crate::user::UserData;
use serde::{Deserialize, Serialize};

/// Basic structure of a Reddit response.
/// See: <https://github.com/reddit-archive/reddit/wiki/JSON>
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum RedditThing {
    #[serde(rename = "t1")]
    Comment(CommentData),
    #[serde(rename = "t2")]
    User(UserData),
    #[serde(rename = "t3")]
    Submission(SubmissionData),
    #[serde(rename = "t5")]
    Subreddit(SubredditData),
    #[serde(rename = "Listing")]
    Listing(ListingData<RedditThing>),
}
