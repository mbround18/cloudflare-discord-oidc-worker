use worker::{FormEntry, Headers, Method, Request, RequestInit, Response, RouteContext};

use crate::{
    config::Config,
    discord::{client::fetch_discord_data, models::*},
    error::AppError,
    jwt::claims::CustomClaims,
};

pub async fn token_handler(
    mut req: Request,
    ctx: RouteContext<Config>,
) -> Result<Response, AppError> {
    let config = &ctx.data;

    let form_data = req
        .form_data()
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let code = match form_data.get("code") {
        Some(FormEntry::Field(code)) if !code.is_empty() => code,
        _ => {
            return Response::error("Missing code.", 400)
                .map_err(|e| AppError::Validation(e.to_string()))
        }
    };

    // Exchange code for token
    let token_url = format!("{}/oauth2/token", Config::discord_api_base());
    let mut params = vec![
        ("client_id", config.client_id.clone()),
        ("client_secret", config.client_secret.clone()),
        ("redirect_uri", config.redirect_url.clone()),
        ("code", code.clone()),
        ("grant_type", "authorization_code".to_string()),
        ("scope", "identify email".to_string()),
    ];

    if let Some(FormEntry::Field(verifier)) = form_data.get("code_verifier") {
        params.push(("code_verifier", verifier.to_string()));
    }

    let mut headers = Headers::new();
    headers
        .set("Content-Type", "application/x-www-form-urlencoded")
        .map_err(|e| AppError::Unexpected(e.to_string()))?;

    let encoded_params =
        serde_urlencoded::to_string(&params).map_err(|e| AppError::Unexpected(e.to_string()))?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_headers(headers);
    init.with_body(Some(encoded_params.into()));

    let request = Request::new_with_init(&token_url, &init)
        .map_err(|e| AppError::Unexpected(e.to_string()))?;
    let token_resp: DiscordTokenResponse = fetch_discord_data(request).await?;

    // Fetch user info
    let user_info: DiscordUserInfo =
        fetch_discord_data(discord_get("/users/@me", &token_resp.access_token)?).await?;

    if !user_info.verified.unwrap_or(false) {
        return Response::error("User not verified.", 400)
            .map_err(|e| AppError::Validation(e.to_string()));
    }

    // Fetch guilds
    let guilds: Vec<DiscordGuild> =
        fetch_discord_data(discord_get("/users/@me/guilds", &token_resp.access_token)?)
            .await
            .unwrap_or_default();

    let guild_ids: Vec<String> = guilds.into_iter().map(|g| g.id).collect();

    let preferred_username = if user_info.discriminator != "0" {
        format!("{}#{}", user_info.username, user_info.discriminator)
    } else {
        user_info.username.clone()
    };
    let display_name = user_info
        .global_name
        .clone()
        .unwrap_or_else(|| user_info.username.clone());

    let jwt_claims = CustomClaims::from_user_info(
        &user_info,
        preferred_username,
        display_name,
        guild_ids,
        &config.issuer,
        &config.client_id,
    );

    let key = crate::jwt::signer::load_or_generate_private_key(&ctx.env).await?;

    let id_token = crate::jwt::signer::generate_jwt_with_key(&key, jwt_claims)?;

    let mut response_map =
        serde_json::to_value(token_resp).map_err(|e| AppError::Unexpected(e.to_string()))?;
    if let Some(obj) = response_map.as_object_mut() {
        obj.insert("scope".into(), "identify email".into());
        obj.insert("id_token".into(), id_token.into());
    }

    Response::from_json(&response_map).map_err(|e| AppError::Unexpected(e.to_string()))
}

fn discord_get(endpoint: &str, access_token: &str) -> Result<Request, AppError> {
    let url = format!("{}{}", Config::discord_api_base(), endpoint);
    let mut headers = Headers::new();
    headers
        .set("Authorization", &format!("Bearer {}", access_token))
        .map_err(|e| AppError::Unexpected(e.to_string()))?;

    let mut init = RequestInit::new();
    init.with_method(Method::Get);
    init.with_headers(headers);

    Request::new_with_init(&url, &init).map_err(|e| AppError::Unexpected(e.to_string()))
}
