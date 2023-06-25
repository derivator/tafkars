use crate::subreddit;
use serde::{Deserialize, Serialize};
use subreddit::SubredditData;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(tag = "kind", rename = "t2")]
pub struct User {
    pub data: UserData,
}

/// AboutData
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserData {
    /// Is employee
    pub is_employee: Option<bool>,
    /// has visited new profile
    pub has_visited_new_profile: Option<bool>,
    /// is friend
    pub is_friend: Option<bool>,
    /// pref no profanity
    pub pref_no_profanity: Option<bool>,
    /// has external account
    pub has_external_account: Option<bool>,
    /// pref geopopoular
    pub pref_geopopular: Option<String>,
    /// pref show trending
    pub pref_show_trending: Option<bool>,
    /// subreddit
    pub subreddit: Option<SubredditData>,
    /// pref show presence
    pub pref_show_presence: Option<bool>,
    /// snoovatar img
    pub snoovatar_img: Option<String>,
    /// snoovatar size. Array of size 2
    pub snoovatar_size: Option<[u64; 2]>,
    /// gold expiration
    pub gold_expiration: Option<String>,
    /// has gold subscription
    pub has_gold_subscription: Option<bool>,
    /// is sponsor
    pub is_sponsor: Option<bool>,
    /// num friends
    pub num_friends: Option<i32>,
    /// can edit name
    pub can_edit_name: Option<bool>,
    /// is blocked
    pub is_blocked: Option<bool>,
    /// verified
    pub verified: Option<bool>,
    /// new modmail exists
    pub new_modmail_exists: Option<bool>,
    /// pref autoplay
    pub pref_autoplay: Option<bool>,
    /// coints
    pub coins: Option<i32>,
    /// has paypal subscription
    pub has_paypal_subscription: Option<bool>,
    /// has subscribed to premium
    pub has_subscribed_to_premium: Option<bool>,
    /// id
    pub id: Option<String>,
    /// can create subreddit
    pub can_create_subreddit: Option<bool>,
    /// over 18
    pub over_18: Option<bool>,
    /// is gold
    pub is_gold: Option<bool>,
    /// is mod
    pub is_mod: Option<bool>,
    /// awarded karma
    pub awarder_karma: Option<i32>,
    /// awardee karma
    pub awardee_karma: Option<i32>,
    /// link karma
    pub link_karma: Option<i32>,
    /// comment karma
    pub comment_karma: Option<i32>,
    /// total karma
    pub total_karma: Option<i32>,
    /// suspension expiration utc
    pub suspension_expiration_utc: Option<i64>,
    /// has stripe subscription
    pub has_stripe_subscription: Option<bool>,
    /// is suspended
    pub is_suspended: Option<bool>,
    /// pref video autopaly
    pub pref_video_autoplay: Option<bool>,
    /// has android subscription
    pub has_android_subscription: Option<bool>,
    /// in redesign beta
    pub in_redesign_beta: Option<bool>,
    /// icon img
    pub icon_img: Option<String>,
    /// has mod mail
    pub has_mod_mail: Option<bool>,
    /// pref nightmode
    pub pref_nightmode: Option<bool>,
    /// hide from robots
    pub hide_from_robots: Option<bool>,
    /// password set
    pub password_set: Option<bool>,
    /// modhash
    pub modhash: Option<String>,
    /// force password reset
    pub force_password_reset: Option<bool>,
    /// inbox count
    pub inbox_count: Option<i32>,
    /// pref top karma subreddits
    pub pref_top_karma_subreddits: Option<bool>,
    /// has mail
    pub has_mail: Option<bool>,
    /// pref show snoovatar
    pub pref_show_snoovatar: Option<bool>,
    /// name
    pub name: Option<String>,
    /// pref clickgadget
    pub pref_clickgadget: Option<i32>,
    /// created
    pub created: Option<f64>,
    /// has verified email
    pub has_verified_email: Option<bool>,
    /// gold creddits
    pub gold_creddits: Option<i32>,
    /// created utc
    pub created_utc: Option<f64>,
    /// has ios subscription
    pub has_ios_subscription: Option<bool>,
    /// pref show twitter
    pub pref_show_twitter: Option<bool>,
    /// in beta
    pub in_beta: Option<bool>,
    /// accept followers
    pub accept_followers: Option<bool>,
    /// has subscribed
    pub has_subscribed: Option<bool>,
    /// accept pms
    pub accept_pms: Option<bool>,
}
