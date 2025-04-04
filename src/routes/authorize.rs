use crate::{config::Config, error::AppError};
use worker::{Headers, Request, Response, RouteContext};

pub async fn authorize_handler(
    req: Request,
    ctx: RouteContext<Config>,
) -> Result<Response, AppError> {
    let config = &ctx.data;

    let scopemode = ctx
        .param("scopemode")
        .map(|m| m.to_string())
        .unwrap_or_default();

    if !Config::valid_scopemodes().contains(&scopemode.as_str()) {
        return Response::error("Invalid scopemode", 400)
            .map_err(|e| AppError::Validation(e.to_string()));
    }

    let url = req.url().map_err(|e| AppError::Unexpected(e.to_string()))?;
    let query_pairs = url.query_pairs();

    let mut client_id_q = String::new();
    let mut redirect_uri_q = String::new();
    let mut state = String::new();

    for (k, v) in query_pairs {
        match k.as_ref() {
            "client_id" => client_id_q = v.into_owned(),
            "redirect_uri" => redirect_uri_q = v.into_owned(),
            "state" => state = v.into_owned(),
            _ => {}
        }
    }

    if client_id_q != config.client_id || redirect_uri_q != config.redirect_url {
        return Response::error("Bad request", 400)
            .map_err(|e| AppError::Validation(e.to_string()));
    }

    let scope = if scopemode == "guilds" {
        "identify email guilds"
    } else {
        "identify email"
    };

    let mut params = vec![
        ("client_id", config.client_id.as_str()),
        ("redirect_uri", config.redirect_url.as_str()),
        ("response_type", "code"),
        ("scope", scope),
        ("state", state.as_str()),
        ("prompt", "none"),
    ];

    // Optional PKCE fields
    let mut code_challenge = None;
    let mut code_challenge_method = None;

    for (k, v) in url.query_pairs() {
        match k.as_ref() {
            "code_challenge" => code_challenge = Some(v.into_owned()),
            "code_challenge_method" => code_challenge_method = Some(v.into_owned()),
            _ => {}
        }
    }

    if let Some(challenge) = &code_challenge {
        params.push(("code_challenge", challenge.as_str()));
    }
    if let Some(method) = &code_challenge_method {
        params.push(("code_challenge_method", method.as_str()));
    }

    let qs =
        serde_urlencoded::to_string(params).map_err(|e| AppError::Unexpected(e.to_string()))?;
    let redirect_url = format!("https://discord.com/oauth2/authorize?{}", qs);

    let mut headers = Headers::new();
    headers
        .set("Location", &redirect_url)
        .map_err(|e| AppError::Unexpected(e.to_string()))?;

    Ok(Response::empty()
        .map_err(|e| AppError::Unexpected(e.to_string()))?
        .with_status(302)
        .with_headers(headers))
}
