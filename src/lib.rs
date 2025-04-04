use crate::{
    config::Config,
    error::IntoWorkerError,
    routes::{authorize::authorize_handler, jwks::jwks_handler, token::token_handler},
    utils::logging::setup_panic_hook,
};
use worker::*;

mod config;
mod discord;
mod error;
mod jwt;
mod routes;
mod utils;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    setup_panic_hook();

    let config = Config::from_env(&env).unwrap_or_else(|e| {
        log::error!("Failed to load config: {}", e);
        panic!("Failed to load config");
    });

    Router::with_data(config)
        .get_async("/authorize/:scopemode", |req, ctx| async move {
            authorize_handler(req, ctx).await.into_worker_error()
        })
        .post_async("/token", |req, ctx| async move {
            token_handler(req, ctx).await.into_worker_error()
        })
        .get_async("/jwks.json", |req, ctx| async move {
            jwks_handler(req, ctx).await.into_worker_error()
        })
        .run(req, env)
        .await
}
