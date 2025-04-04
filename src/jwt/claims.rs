use crate::discord::models::DiscordUserInfo;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
pub struct CustomClaims {
    pub sub: String,                // ✅ REQUIRED: unique user ID
    pub iss: String,                // ✅ REQUIRED: token issuer
    pub aud: String,                // ✅ REQUIRED: token audience
    pub exp: usize,                 // ✅ REQUIRED: expiration time (unix)
    pub email: Option<String>,      // ⚠️ REQUIRED for Access if using email auth
    pub preferred_username: String, // Optional but often helpful
    pub global_name: Option<String>,
    pub name: String,
    pub guilds: Vec<String>,
    #[serde(flatten)]
    pub extra: Value,
}

impl CustomClaims {
    pub fn from_user_info(
        user: &DiscordUserInfo,
        preferred_username: String,
        display_name: String,
        guilds: Vec<String>,
        issuer: &str,
        audience: &str,
    ) -> Self {
        let now = (js_sys::Date::now() / 1000.0) as usize;
        let exp = now + 3600;

        let mut extra_map = Map::new();
        extra_map.insert("id".to_string(), Value::String(user.id.clone()));
        extra_map.insert("username".to_string(), Value::String(user.username.clone()));
        extra_map.insert(
            "discriminator".to_string(),
            Value::String(user.discriminator.clone()),
        );

        Self {
            sub: user.id.clone(), // 👈 UNIQUE USER IDENTIFIER
            iss: issuer.to_string(),
            aud: audience.to_string(),
            exp,
            email: user.email.clone(),
            preferred_username,
            global_name: user.global_name.clone(),
            name: display_name,
            guilds,
            extra: Value::Object(extra_map),
        }
    }
}
