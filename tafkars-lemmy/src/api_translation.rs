use crate::ResponseState;
use lemmy_api_common::comment::GetCommentsResponse;
use lemmy_api_common::lemmy_db_schema::newtypes::DbUrl;
use lemmy_api_common::lemmy_db_views::structs::{CommentView, PostView};
use lemmy_api_common::post::GetPostsResponse;
use serde_json::Value;
use std::borrow::ToOwned;
use tafkars::comment::{Comment, CommentData, MaybeReplies};
use tafkars::listing::{Listing, ListingData};
use tafkars::submission::{Submission, SubmissionData};

use markdown;

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
}

pub fn posts(state: &ResponseState, res: GetPostsResponse) -> Listing<Submission> {
    let posts = res.posts.into_iter().map(|p| post(state, p)).collect();

    Listing {
        data: ListingData {
            modhash: Some("c2swiur5ry66d67eca991e911ebb57b824a27f0d9ad1264bf6".to_string()),
            dist: Some(1),
            after: None,
            before: None,
            children: posts,
        },
    }
}

pub fn post(state: &ResponseState, pv: PostView) -> Submission {
    let p = pv.post;
    let community_id = p.community_id.0;
    let post_id = p.id;
    let subreddit = state
        .escape_actor_id(&pv.community.actor_id)
        .unwrap_or("invalid".to_owned());
    let thumbnail = p
        .thumbnail_url
        .map(|u| u.to_string())
        .unwrap_or("self".to_owned());

    let edited = if let Some(timestamp) = p.updated {
        Value::from(timestamp.timestamp() as f64)
    } else {
        Value::from(false)
    };
    let author = state
        .escape_actor_id(&pv.creator.actor_id)
        .unwrap_or("invalid".to_owned());

    let permalink = format!("/comments/{post_id}/"); // TODO: this might work for some clients, but reddit does /r/{subreddit}/comments/{id}/{urlsafe_name}

    Submission {
        data: SubmissionData {
            domain: Some(format!("self.{subreddit}")),
            banned_by: None,
            subreddit,
            selftext_html: None,
            selftext: p.body.unwrap_or("".to_owned()),
            likes: pv.my_vote.map(|v| v > 0),
            suggested_sort: None,
            link_flair_text: None,
            id: post_id.to_string(),
            gilded: 0,
            archived: false,
            clicked: false,
            author,
            score: pv.counts.score,
            approved_by: None,
            over_18: p.nsfw,
            spoiler: false,
            hidden: false,
            num_comments: pv.counts.comments as u64,
            thumbnail,
            subreddit_id: format!("t5_{community_id}"),
            hide_score: false,
            edited,
            link_flair_css_class: None,
            author_flair_css_class: None,
            downs: pv.counts.downvotes,
            ups: pv.counts.upvotes,
            upvote_ratio: pv.counts.upvotes as f64 / pv.counts.downvotes as f64,
            saved: false,
            removal_reason: None,
            stickied: false,
            is_self: p.url.is_none(),
            permalink,
            locked: p.locked,
            name: format!("t3_{post_id}"),
            created: p.published.timestamp() as f64,
            url: p.url.map(|u| u.to_string()),
            author_flair_text: None,
            quarantine: false,
            title: p.name,
            created_utc: p.published.timestamp() as f64, // TODO: wrong?
            distinguished: None,
            visited: false,
            num_reports: None,
            is_video: false,
            can_mod_post: false,
        },
    }
}

/// Insert `comment` into the comment tree at a position specified by the ancestor ids in `path`
pub fn insert_at(comments: &mut Vec<Comment>, path: &[String], comment: Comment) {
    if path.is_empty() {
        comments.push(comment);
        return;
    }

    // We paid for the bandwidth, show the damn comment even if the ancestor has been cruelly ripped from us by pagination
    // TODO: alert the user to missing ancestors somehow, e.g. by creating pseudo-ancestors
    for parent_id in path {
        if let Some(parent) = comments
            .iter_mut()
            .find(|c| c.data.id.as_ref().unwrap() == parent_id)
        {
            insert_at(
                parent
                    .data
                    .replies
                    .get_or_insert(Default::default())
                    .replies(),
                &path[1..],
                comment,
            );

            return;
        }
    }
    comments.push(comment);
}

pub fn comments(state: &ResponseState, mut res: GetCommentsResponse) -> Listing<Comment> {
    let depth = |cv: &CommentView| cv.comment.path.matches('.').count();
    res.comments.sort_by_key(|cv| depth(cv)); // stable sort preserves Hot/Old/New/... sorting

    let mut comments: Vec<Comment> = Vec::new();
    for cv in res.comments.into_iter() {
        let mut path: Vec<String> = cv.comment.path.split('.').map(|s| s.to_owned()).collect();
        path.pop();
        insert_at(&mut comments, &path[1..], comment(state, cv))
    }

    Listing {
        data: ListingData {
            modhash: Some("c2swiur5ry66d67eca991e911ebb57b824a27f0d9ad1264bf6".to_string()),
            dist: Some(1),
            after: None,
            before: None,
            children: comments,
        },
    }
}

pub fn comment(state: &ResponseState, cv: CommentView) -> Comment {
    let c = cv.comment;
    let author = state
        .escape_actor_id(&cv.creator.actor_id)
        .unwrap_or("invalid".to_owned());
    let id = c.id.0.to_string();

    let body = if c.deleted {
        "[deleted]".to_owned()
    } else {
        c.content
    };

    let mut body_html = markdown::to_html(&body);
    if !state.res_config.raw_json {
        body_html = html_escape::encode_safe(&body_html).to_string();
    }

    let path: Vec<&str> = c.path.split('.').collect();
    let parent_id = *path.last().unwrap_or(&"wtf");
    let parent_id = if parent_id == "0" {
        format!("t3_{}", cv.post.id)
    } else {
        format!("t1_{parent_id}")
    };
    Comment {
        data: CommentData {
            total_awards_received: None,
            approved_at_utc: None,
            link_id: None,
            author_flair_template_id: None,
            likes: None,
            saved: Some(false),
            id: Some(id.clone()),
            gilded: Some(0),
            archived: Some(false),
            no_follow: None,
            author: Some(author),
            can_mod_post: Some(false),
            created_utc: Some(c.published.timestamp() as f64), //TODO: wrong?
            send_replies: None,
            parent_id: Some(parent_id),
            score: Some(cv.counts.score as i32),
            author_fullname: None,
            over_18: None,
            approved_by: None,
            subreddit_id: None,
            body: Some(body),
            link_title: None,
            name: Some(format!("t1_{id}")),
            author_patreon_flair: None,
            downs: Some(cv.counts.downvotes as i32),
            is_submitter: None,
            body_html: Some(body_html),
            distinguished: None,
            stickied: Some(false),
            author_premium: None,
            can_gild: None,
            subreddit: None,
            author_flair_text_color: None,
            score_hidden: Some(false),
            permalink: None,
            num_reports: None,
            link_permalink: None,
            link_author: None,
            subreddit_name_prefixed: None,
            author_flair_text: None,
            link_url: None,
            created: None,
            collapsed: None,
            controversiality: Some(0),
            locked: Some(false),
            quarantine: None,
            subreddit_type: None,
            ups: Some(cv.counts.upvotes as i32),
            replies: Some(MaybeReplies::Str("".to_owned())),
        },
    }
}
