use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscordTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DiscordUserInfo {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub email: Option<String>,
    pub verified: Option<bool>,
    pub global_name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct DiscordGuild {
    pub id: String,
    // #[serde(default)]
    // pub name: String,
}
