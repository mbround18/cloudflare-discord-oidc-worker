use crate::error::AppError;
use crate::jwt::claims::CustomClaims;
use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey, LineEnding};
use rsa::RsaPrivateKey;
use worker::Env;

const PRIVATE_KEY_KV: &str = "JWT_PRIVATE_KEY";

/// Load or generate an RSA private key from KV
pub async fn load_or_generate_private_key(env: &Env) -> Result<RsaPrivateKey, AppError> {
    let kv = env
        .kv("KEYS_STORE")
        .map_err(|e| AppError::Unexpected(e.to_string()))?;

    if let Some(pem) = kv
        .get(PRIVATE_KEY_KV)
        .text()
        .await
        .map_err(|e| AppError::Unexpected(e.to_string()))?
    {
        RsaPrivateKey::from_pkcs1_pem(&pem).map_err(|e| AppError::Keygen(e.to_string()))
    } else {
        let mut rng = rand::rngs::OsRng;
        let key =
            RsaPrivateKey::new(&mut rng, 8192).map_err(|e| AppError::Keygen(e.to_string()))?;
        let pem = key
            .to_pkcs1_pem(LineEnding::CRLF)
            .map_err(|e| AppError::Keygen(e.to_string()))?
            .to_string();
        kv.put(PRIVATE_KEY_KV, pem)
            .map_err(|e| AppError::Unexpected(e.to_string()))?
            .execute()
            .await
            .map_err(|e| AppError::Unexpected(e.to_string()))?;
        Ok(key)
    }
}

/// Sign JWT using a provided RSA key (to avoid reloading from KV)
pub fn generate_jwt_with_key(
    key: &RsaPrivateKey,
    claims: CustomClaims,
) -> Result<String, AppError> {
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use rsa::pkcs1::EncodeRsaPrivateKey;
    use rsa::pkcs1::LineEnding;

    let pem = key
        .to_pkcs1_pem(LineEnding::CRLF)
        .map_err(|e| AppError::Keygen(e.to_string()))?
        .to_string();

    let encoding_key =
        EncodingKey::from_rsa_pem(pem.as_bytes()).map_err(|e| AppError::Jwt(e.to_string()))?;

    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some("jwtRS256".into());

    encode(&header, &claims, &encoding_key).map_err(|e| AppError::Jwt(e.to_string()))
}
