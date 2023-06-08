//! # Subreddit Comment Responses
use crate::listing::{Listing, ListingData};
use serde::{Deserialize, Serialize};

/// SubredditCommentsData  
/// Everything is an option to deal with both `latest_comments` and `article_comments`
#[derive(Serialize, Debug, Deserialize)]
pub struct CommentData {
    /// Total awards
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_awards_received: Option<i32>,
    /// Approved at (UTC)
    pub approved_at_utc: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    /// What is this
    pub author_flair_template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saved: Option<bool>,
    /// ID
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gilded: Option<i32>,
    /// Archived
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// No follow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_follow: Option<bool>,
    /// Auuthor
    pub author: Option<String>,
    /// Can mod post
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_mod_post: Option<bool>,
    /// Created (UTC)
    pub created_utc: Option<f64>,
    /// Send replies
    pub send_replies: Option<bool>,
    /// Parent ID
    pub parent_id: Option<String>,
    /// Score
    pub score: Option<i32>,
    /// Author fullname
    pub author_fullname: Option<String>,
    /// Over 18
    pub over_18: Option<bool>,
    /// Approved by
    pub approved_by: Option<String>,
    /// Subreddit ID
    pub subreddit_id: Option<String>,
    /// Body
    pub body: Option<String>,
    /// Link title
    pub link_title: Option<String>,
    /// Name
    pub name: Option<String>,
    /// Patreon flair
    pub author_patreon_flair: Option<bool>,
    /// Downs?
    pub downs: Option<i32>,
    /// Is submitter
    pub is_submitter: Option<bool>,
    /// HTML
    pub body_html: Option<String>,
    /// Distinguished
    pub distinguished: Option<String>,
    /// Stickied
    pub stickied: Option<bool>,
    /// Premium
    pub author_premium: Option<bool>,
    /// Can gild
    pub can_gild: Option<bool>,
    /// Subreddit
    pub subreddit: Option<String>,
    /// Flair color
    pub author_flair_text_color: Option<String>,
    /// Score hidden
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_hidden: Option<bool>,
    /// Permalink
    pub permalink: Option<String>,
    /// Number of reports
    pub num_reports: Option<i32>,
    /// Permalink
    pub link_permalink: Option<String>,
    /// Author link
    pub link_author: Option<String>,
    /// Sub name
    pub subreddit_name_prefixed: Option<String>,
    /// Author flair
    pub author_flair_text: Option<String>,
    /// Link url
    pub link_url: Option<String>,
    /// Created
    pub created: Option<f64>,
    /// Collapsed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsed: Option<bool>,
    /// Controversiality
    pub controversiality: Option<i32>,
    /// Locked
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// Quarantine
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quarantine: Option<bool>,
    /// Subreddit type
    pub subreddit_type: Option<String>,
    /// UPS?
    pub ups: Option<i32>,
    /// Replies
    pub replies: Option<MaybeReplies>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename = "t1")]
pub struct Comment {
    pub data: CommentData,
}
// TODO: this is super ugly and should just be Option<Listing<Comment>> with serde tricks to use "" as None
/// Replies can be more comments or an empty string
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeReplies {
    /// Reply
    Reply(Listing<Comment>),
    /// String
    Str(String),
}

impl MaybeReplies {
    pub fn replies(&mut self) -> &mut Vec<Comment> {
        match self {
            MaybeReplies::Reply(l) => &mut l.data.children,
            MaybeReplies::Str(_) => {
                *self = MaybeReplies::Reply(Listing {
                    data: ListingData {
                        modhash: None,
                        dist: None,
                        after: None,
                        before: None,
                        children: vec![],
                    },
                });
                match self {
                    MaybeReplies::Reply(l) => &mut l.data.children,
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl Default for MaybeReplies {
    fn default() -> Self {
        MaybeReplies::Str("".to_owned())
    }
}
