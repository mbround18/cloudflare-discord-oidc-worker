use crate::error::AppError;
use crate::jwt::signer::load_or_generate_private_key;
use rsa::RsaPublicKey;
use worker::{Request, Response, RouteContext};

use base64_url::encode as b64url;
use rsa::traits::PublicKeyParts;
use serde_json::json;

pub async fn jwks_handler(
    _req: Request,
    ctx: RouteContext<crate::config::Config>,
) -> Result<Response, AppError> {
    let env = &ctx.env;

    let private_key = load_or_generate_private_key(env).await?;
    let public_key = RsaPublicKey::from(&private_key);

    let n = b64url(&public_key.n().to_bytes_be());
    let e = b64url(&public_key.e().to_bytes_be());

    let jwk = json!({
        "keys": [{
            "alg": "RS256",
            "kty": "RSA",
            "use": "sig",
            "kid": "jwtRS256",
            "n": n,
            "e": e
        }]
    });

    Response::from_json(&jwk).map_err(|e| AppError::Unexpected(e.to_string()))
}
