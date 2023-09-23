use logger_rs::error;
use reqwest::header;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Role {
    id: String,
    name: String,
    description: Option<String>,
    permissions: String,
    position: i32,
    color: i32,
    hoist: bool,
    managed: bool,
    mentionable: bool,
    icon: Option<String>,
    unicode_emoji: Option<String>,
    flags: i32,
    tags: Option<Tags>,
}
#[derive(Serialize, Deserialize)]
struct Tags {
    bot_id: String,
}
#[derive(Serialize, Deserialize)]
pub struct GuildData {
    id: String,
    name: String,
    icon: String,
    description: Option<String>,
    home_header: Option<String>,
    splash: Option<String>,
    discovery_splash: Option<String>,
    features: Vec<String>,
    banner: Option<String>,
    owner_id: String,
    application_id: Option<String>,
    region: String,
    afk_channel_id: Option<String>,
    afk_timeout: i32,
    system_channel_id: String,
    system_channel_flags: i32,
    widget_enabled: bool,
    widget_channel_id: Option<String>,
    verification_level: i32,
    roles: Vec<Role>,
    default_message_notifications: i32,
    mfa_level: i32,
    explicit_content_filter: i32,
    max_presences: Option<String>,
    max_members: i32,
    max_stage_video_channel_users: i32,
    max_video_channel_users: i32,
    vanity_url_code: Option<String>,
    premium_tier: i32,
    premium_subscription_count: i32,
    preferred_locale: String,
    rules_channel_id: Option<String>,
    safety_alerts_channel_id: Option<String>,
    public_updates_channel_id: Option<String>,
    hub_type: Option<String>,
    premium_progress_bar_enabled: bool,
    latest_onboarding_question_id: Option<String>,
    nsfw: bool,
    nsfw_level: i32,
    emojis: Vec<String>,
    stickers: Vec<String>,
    incidents_data: Option<String>,
    inventory_settings: Option<String>,
    embed_enabled: bool,
    embed_channel_id: Option<String>,
}

impl GuildData {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

// pub enum ReqError {
// ReqwestError(reqwest::Error),
// OtherError(String),
// }

// impl From<reqwest::Error> for ReqError {
// fn from(error: reqwest::Error) -> Self {
// ReqError::ReqwestError(error)
// }
// }

pub async fn guild(token: String, id: String) -> Option<GuildData> {
    let url = format!("https://discord.com/api/v10/guilds/{}", &id);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(header::AUTHORIZATION, &token)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        error!("Error: {}", response.status());
        return None;
    }

    let guild: GuildData = response.json().await.unwrap();
    return Some(guild);
}
