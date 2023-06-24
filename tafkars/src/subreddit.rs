//! # Subreddit Responses
use crate::submission;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(tag = "kind", rename = "t5")]
pub struct Subreddit {
    pub data: SubredditData,
}

/// accounts_active and active_user_count fields in `SubredditData`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountsActive {
    /// The (approximate) number of users interacting with this subreddit over the past 15 minutes.
    /// this value may be "fuzzed" (see accounts_active_is_fuzzed).
    Number(u64),
    /// Reddit occasionally returns an empty array instead of an integer.
    Vector(Vec<u8>),
}

/// If the API user has user flair in this subreddit, and its user_flair_type is richtext,
/// this will be an array containing two string elements which define the user's flair.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct RichtextFlair {
    /// contains the string text
    e: String,
    /// contains the literal string that comprises the user's flair.
    t: String,
}

/// SubredditData
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SubredditData {
    /// The HTML hex code of the API user's flair background color in this subreddit,
    /// if any. If the API user has no flair, or no background color is defined, this will be null.
    pub user_flair_background_color: Option<String>,
    /// This subreddit's configured "text to show on submission page," if any. If text is not configured,
    /// an empty string is returned.
    pub submit_text: Option<String>,
    //// The contents of submit_text converted to HTML entities.
    /// If no text has been configured, this will be null.
    pub submit_text_html: Option<String>,
    /// The text configured for this subreddit's "Custom label for submit text post button" option,
    /// if any. If no text has been configured, this will be null.
    pub submit_text_label: Option<String>,
    /// Whether or not submitting new links is restricted in this subreddit.
    /// This will be true when subreddit_type is restricted.
    pub restrict_posting: Option<bool>,
    /// Whether or not the API user is banned from participating in this subreddit.
    pub user_is_banned: Option<bool>,
    /// Whether or not this subreddit has the "allow free-form reports by users" option enabled.
    /// If the option is disabled, users must choose from a list of canned reporting reasons.
    pub free_form_reports: Option<bool>,
    /// Whether or not the API user has access to edit this subreddit's wiki.
    /// This will be true if one or more of the following conditions is satisfied:
    /// - The wiki is set to be editable by "anyone"
    /// - The wiki is set to "disabled" and the API user is a moderator
    /// - The wiki is set to "mod editing" and the API user is either a moderator or an approved contributor
    /// - Otherwise, this will be null.
    pub wiki_enabled: Option<bool>,
    /// Whether or not the API user has been muted in this subreddit.
    pub user_is_muted: Option<bool>,
    /// Whether or not the API user is allowed to set user flair in this subreddit.
    /// This will be true if:
    /// - the "Allow users to assign their own" user flair option is enabled
    /// - the API user is a moderator of this subreddit with the "flair" permission.
    /// - otherwise this will be null.
    pub user_can_flair_in_sr: Option<bool>,
    /// The base name of the subreddit with no decorators or adornments, e.g. funny.
    pub display_name: Option<String>,
    /// The name of the subreddit prefixed with its relative URI path, e.g. r/funny, or u/joe for user profile subreddits.
    pub display_name_prefixed: Option<String>,
    /// The URL to this subreddit's custom header image, if any.
    /// In the legacy web interface, this is the image that would appear in place of the default Snoo.
    /// If no custom header image is configured, this will be null.
    pub header_img: Option<String>,
    /// The title of this subreddit.
    pub title: Option<String>,
    /// Unknown
    pub allow_galleries: Option<bool>,
    /// If an icon_img has been configured for this subreddit,
    /// this will be an array containing two integer elements that define the image's width and height, e.g. `[256, 256]`.
    /// If no icon image is configured, this will be null.
    pub icon_size: Option<Vec<u64>>,
    /// The base36 internal Reddit identifier for this subreddit, e.g. 2qpqw.
    pub id: Option<String>,
    /// The HTML hex color code for this subreddit's primary color, if any. This corresponds to the "Base" theme color in the redesign web interface.
    /// If a color is not configured, an empty string is returned.
    pub primary_color: Option<String>,
    /// The (approximate) number of users interacting with this subreddit over the past 15 minutes.
    /// this value may be "fuzzed" (see accounts_active_is_fuzzed).
    /// Reddit occasionally returns an empty array instead of an integer. This isn't reproducible.
    /// This field is a duplicate of active_user_count.
    pub accounts_active: Option<AccountsActive>,
    /// The (approximate) number of users interacting with this subreddit over the past 15 minutes.
    /// this value may be "fuzzed" (see accounts_active_is_fuzzed).
    /// Reddit occasionally returns an empty array instead of an integer. This isn't reproducible.
    /// This field is a duplicate of accounts_active.
    pub active_user_count: Option<AccountsActive>,
    /// The URL to this subreddit's icon image, if any. In the redesign web interface,
    /// this icon is displayed in the "Community Details" portion of the sidebar, and on site listings (e.g. front page, /r/all)
    /// when the card layout is selected. If no icon image is configured, an empty string is returned.
    pub icon_img: Option<String>,
    /// Whether or not this subreddit exposes its traffic statistics to the public.
    pub public_traffic: Option<bool>,
    /// The number of accounts subscribed to this subreddit.
    pub subscribers: Option<u64>,
    /// If the API user has user flair in this subreddit, and its user_flair_type is richtext, this will be an array
    /// containing two string elements which define the user's flair. The first element, named e, contains the string text.
    /// The second element, named t, contains the literal string that comprises the user's flair. For example:
    /// [{"e":"text","t":"Proud Shitposter"}]
    pub user_flair_richtext: Option<Vec<RichtextFlair>>,
    /// Unknown. If set, the value appears to max out at 100.
    pub videostream_links_count: Option<u8>,
    /// The fullname identifier of this subreddit. This is a combination of the thing kind (t5) and the id,
    /// separated by an underscore, e.g. t5_2qh38
    pub name: Option<String>,
    /// Whether or not this subreddit is quarantined.
    /// This is a restricted property. To access a quarantined subreddit through this endpoint,
    /// the API user must have affirmatively clicked "continue" on its quarantine interstitial page at some point,
    /// and must be authenticated to the API with a valid access token.
    /// Other users will receive a 403 error when attempting to access a quarantined subreddit.
    pub quarantine: Option<bool>,
    /// Whether or not ads have been administratively suppressed in this subreddit.
    pub hide_ads: Option<bool>,
    /// Whether or not this subreddit has the "Enable emojis in this community" option enabled.
    pub emojis_enabled: Option<bool>,
    /// The advertiser categeory this subreddit falls under, if any has been assigned. If no value has been assigned,
    /// an empty string is returned.
    /// Observed values (as of August 2019) include:
    /// - Automotive
    /// - Business / Finance
    /// - College / University
    /// - Entertainment
    /// - Family & Youth
    /// - Games
    /// - Health
    /// - Lifestyles
    /// - Local
    /// - Retail
    /// - Sports
    /// - Technology
    /// - Travel
    pub advertiser_category: Option<String>,
    /// A description of this subreddit, as supplied by its moderator(s).
    /// If none has been configured, an empty string is returned.
    /// In both the legacy and redesign web interfaces, this text is used to build the <meta name="description"> tag.
    pub public_description: Option<String>,
    /// The "Minutes to hide comment scores" value configured for this subreddit.
    pub comment_score_hide_mins: Option<u64>,
    /// Unknown
    pub allow_predictions: Option<bool>,
    /// Whether or not the API user has added this subreddit to their favorites.
    pub user_has_favorited: Option<bool>,
    /// If the API user has user flair in this subreddit, and the user flair has been chosen from a predefined template,
    /// this will contain the 36-character UUID of the template. If user flair is not configured,
    /// or is an ad-hoc string with no predefined template, this will be null.
    pub user_flair_template_id: Option<bool>,
    /// The URL to this subreddit's community icon image, if any has been configured.
    /// If no value has been assigned, an empty string is returned.
    pub community_icon: Option<String>,
    /// The URL to this subreddit's banner background image, if any. This is the banner that displays on the desktop site.
    /// If no value has been assigned, an empty string is returned.
    pub banner_background_image: Option<String>,
    /// Whether or not this subreddit has the "enable marking posts as Original Content (OC) on the desktop redesign"
    /// option enabled.
    pub original_content_tag_enabled: Option<bool>,
    /// The contents of description converted to HTML entities. If no description has been configured, this will be null.
    pub description_html: Option<String>,
    /// Whether or not this subreddit allows tagging submissions as spoilers.
    pub spoilers_enabled: Option<bool>,
    /// The title configured for this subreddit's header_img, if any.
    /// If no header title is configured, this will be null.
    /// This field may be populated even if header_img is null, as an artifact from that field being previously set.
    pub header_title: Option<String>,
    /// If a header_img has been configured for this subreddit, this will be an array containing two integer elements
    /// that define the image's width and height, e.g. `[16, 16]`. If no header image is configured, this will be null.
    pub header_size: Option<Vec<u64>>,
    /// The CSS position of user flair in this subreddit, relative to the username,
    /// i.e. left or right. If user flair is not configured, an empty string is returned.
    pub user_flair_position: Option<String>,
    /// Whether or not this subreddit has the "mark all posts in this subreddit as
    /// Original Content (OC) on the desktop redesign" option enabled.
    pub all_original_content: Option<bool>,
    /// Whether or not this subreddit has custom menu tabs (submenu or link tabs) defined in the redesign web interface.
    pub has_menu_widget: Option<bool>,
    /// Whether or not this subreddit has enrolled in the new-style modmail interface.
    /// Newly created subreddits are automatically enrolled as of ~2018.
    /// This is a moderator-only property. To receive an accurate value, the API user must be a moderator of the subreddit,
    /// and must be authenticated to the API with a valid access token. Other users will receive a null value.
    pub is_enrolled_in_new_modmail: Option<bool>,
    /// The HTML hex color code for this subreddit's general theme color,
    /// if any is set. This corresponds to the "used as a thematic color for your subreddit on mobile" subreddit option.
    /// If no color is configured, an empty string is returned.
    pub key_color: Option<String>,
    /// Whether or not this subreddit allows users to assign flair to themselves.
    /// If false, only a moderator can assign flair to users.
    pub can_assign_user_flair: Option<bool>,
    /// The unix epoch timestamp at which this subreddit was created.
    /// This is provided as a float, but the fractional part is always zero.
    pub created: Option<f64>,
    /// A numeric value corresponding to the whitelist_status.
    pub wls: Option<u64>,
    /// Whether or not this subreddit has the "expand media previews on comments pages" option enabled.
    pub show_media_preview: Option<bool>,
    /// The type of links that can be submitted in this subreddit. This will typically be one of any, link, or self;
    /// some banned and employee-only subreddits have this value set to an empty string.
    pub submission_type: Option<String>,
    /// Whether or not the API user has subscribed to this subreddit.
    pub user_is_subscriber: Option<bool>,
    /// Whether or not this subreddit has the "Accepting new requests to post" option turned off.
    /// This is intended for subreddits with a subreddit_type of restricted
    pub disable_contributor_requests: Option<bool>,
    /// Unknown
    pub allow_videogifs: Option<bool>,
    /// If the API user has user flair in this subreddit, this will contain the flair type, either text or richtext.
    /// If user flair is not configured, this defaults to text.
    pub user_flair_type: Option<String>,
    /// Unknown
    pub allow_polls: Option<bool>,
    /// Whether or not this subreddit has the "collapse deleted and removed comments" option enabled.
    pub collapse_deleted_comments: Option<bool>,
    /// If "Custom sized emojis" has been configured for this subreddit,
    /// this will be an array containing two integer elements that define the emoji width and height, e.g. `[16, 16]`.
    /// If no custom emoji size is configured, this will be null.
    pub emojis_custom_size: Option<Vec<u64>>,
    /// The contents of public_description converted to HTML entities.
    /// If no public description has been configured, an empty string is returned.
    pub public_description_html: Option<String>,
    /// Whether or not this subreddit has the "allow video uploads" option enabled.
    pub allow_videos: Option<bool>,
    /// Whether or not this subreddit has the "Allow crossposting of posts" preference enabled.
    /// If crossposting is disabled, this will be null instead of false.
    pub is_crosspostable_subreddit: Option<bool>,
    /// The suggested comment sort order for this subreddit, if one has been set.
    /// If no sort order has been configured, this will be null.
    /// Observed values (as of August 2019) include: confidence, controversial, live, new, old, qa, random, top
    pub suggested_comment_sort: Option<submission::SortOrder>,
    /// Whether or not users can assign flair to their own links in this subreddit.
    /// If false, only a moderator can assign flair to links.
    pub can_assign_link_flair: Option<bool>,
    /// For subreddits with a low number of subscribers,
    /// Reddit will artificially inflate the active user count in order to mitigate statistical inference attacks.
    pub accounts_active_is_fuzzed: Option<bool>,
    /// The CSS position of a link's flair in this subreddit, relative to the link, i.e. left or right.
    /// If link flair is not configured, an empty string is returned.
    pub link_flair_position: Option<String>,
    /// Whether or not the API user has opted to display their user flair in this subreddit
    /// This may be null if the API user has no user flair or has not indicated a display preference for it.
    pub user_sr_flair_enabled: Option<bool>,
    /// Whether or not user flair is enabled in this subreddit.
    /// This applies to the subreddit generally, not to the individual API user.
    pub user_flair_enabled_in_sr: Option<bool>,
    /// Whether or not this subreddit has the "allow this subreddit to be exposed to users who have
    /// shown intent or interest through discovery and onboarding" option enabled.
    pub allow_discovery: Option<bool>,
    /// Whether or not the API user allows this subreddit to display custom CSS (legacy terminology) or
    /// a community theme (redesign terminology) via the "allow subreddits to show me custom themes" preference.
    pub user_sr_theme_enabled: Option<bool>,
    /// Whether or not link flair is enabled in this subreddit.
    pub link_flair_enabled: Option<bool>,
    /// The access level applied to this subreddit. This will be one of public, restricted, private, or employees_only.
    /// This is a restricted property. To receive an accurate value for subreddits with a type other than public or restricted,
    /// the API user must have access to the subreddit, and must be authenticated to the API with a valid access token.
    /// Other users will receive a 403 error when attempting to access a private or employees_only subreddit.
    pub subreddit_type: Option<String>,
    /// Unknown. Observed values (as of August 2019) include: low
    /// This is a moderator-only property. To receive an accurate value, the API user must be a moderator of the subreddit,
    /// and must be authenticated to the API with a valid access token. Other users will receive a null value.
    pub notification_level: Option<String>,
    /// The URL to this subreddit's banner background image, if any. This is the banner that displays on the mobile site.
    /// If no value has been assigned, an empty string is returned.
    pub banner_img: Option<String>,
    /// If the API user has user flair in this subreddit, this will contain the literal string that comprises the user's flair.
    /// If user flair is not configured, this will be null.
    pub user_flair_text: Option<String>,
    /// The HTML hex color code for this subreddit's banner background color, if one has been set.
    /// If no value has been assigned, an empty string is returned.
    pub banner_background_color: Option<String>,
    /// Whether or not this subreddit has the "show thumbnail images of content" option enabled.
    pub show_media: Option<bool>,
    /// Whether or not the API user has been explicitly added as an approved user in this subreddit.
    pub user_is_contributor: Option<bool>,
    /// Whether or not this subreddit has the "viewers must be over eighteen years old" option enabled.
    pub over18: Option<bool>,
    /// A description of this subreddit as supplied by its moderator(s).
    pub description: Option<String>,
    /// The text configured for this subreddit's "Custom label for submit link button" option, if any.
    /// If no text has been configured, this will be null.
    pub submit_link_label: Option<String>,
    /// If the API user has user flair in this subreddit, this will contain a contrast label for their flair text,
    /// either light or dark.
    /// If the API user flair is not configured, this will be null.
    pub user_flair_text_color: Option<String>,
    /// Unknown
    pub restrict_commenting: Option<bool>,
    /// The CSS class corresponding to the API user's flair in this subreddit, if any.
    /// If the API user has no flair, or no CSS class is defined, this will be null.
    pub user_flair_css_class: Option<String>,
    /// Whether or not this subreddit has the "allow image uploads and links to image hosting sites" option enabled.
    pub allow_images: Option<bool>,
    /// The language/localization setting for this subreddit, if any. If no language is configured, an empty string is returned.
    pub lang: Option<String>,
    /// The advertising whitelist status of this subreddit, if set.
    /// If no whitelist status has been configured, an empty string is returned.
    /// Observed values (as of August 2019) include: all_ads, house_only, promo_adult_nsfw, promo_all, no_ads
    pub whitelist_status: Option<String>,
    /// The fully-qualified relative URI path to this subreddit, e.g. /r/HotPeppers, or /u/joe for user profile subreddits.
    pub url: Option<String>,
    /// The unix epoch timestamp reflecting the point 8 hours later than created. This is provided as a float,
    /// but the fractional part is always zero.
    pub created_utc: Option<f64>,
    /// If banner_img is set, this will be an array containing two integer elements that define the image's width and height,
    /// e.g. `[1280, 384]`. If no banner image is configured, this will be null.
    pub banner_size: Option<Vec<u64>>,
    /// The URL to this subreddit's mobile banner image. If no mobile banner is configured, an empty string is returned.
    pub mobile_banner_image: Option<String>,
    /// Whether or not the API user is a moderator of this subreddit.
    pub user_is_moderator: Option<bool>,
}

/// Ways to sort submissions in a subreddit
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Hot,
    New,
    Rising,
    Controversial,
    Best,
    Top,
}

/// Time filters for controversial and top sorting
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterTime {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All,
}

#[derive(Debug, Deserialize)]
pub struct Query {
    #[serde(rename = "t")]
    pub time: Option<FilterTime>,
}
