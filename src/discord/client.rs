use crate::error::AppError;
use worker::{Fetch, Request};

/// Fetch and deserialize data from a Discord API request
pub async fn fetch_discord_data<T: for<'de> serde::Deserialize<'de>>(
    request: Request,
) -> Result<T, AppError> {
    let mut resp = Fetch::Request(request)
        .send()
        .await
        .map_err(|e| AppError::DiscordApi(e.to_string()))?;

    if resp.status_code() != 200 {
        return Err(AppError::DiscordApi(format!(
            "Discord API error: status {}",
            resp.status_code()
        )));
    }

    resp.json()
        .await
        .map_err(|e| AppError::DiscordApi(format!("Invalid JSON: {}", e)))
}
