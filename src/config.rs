use worker::{Env, Error};

pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub issuer: String,
}

impl Config {
    pub fn from_env(env: &Env) -> Result<Self, Error> {
        Ok(Self {
            client_id: env.var("DISCORD_CLIENT_ID")?.to_string(),
            client_secret: env.var("DISCORD_CLIENT_SECRET")?.to_string(),
            redirect_url: env.var("DISCORD_REDIRECT_URL")?.to_string(),
            issuer: "https://cloudflare.com".to_string(),
        })
    }

    pub fn valid_scopemodes() -> &'static [&'static str] {
        &["guilds", "email"]
    }

    pub fn discord_api_base() -> &'static str {
        "https://discord.com/api/v10"
    }
}
