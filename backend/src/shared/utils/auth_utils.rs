//! Authentication helpers: password hashing, JWT creation/validation, and refresh token helpers.
//!
//! Exposed utilities:
//! - `hash_password` / `verify_password` — Argon2id password storage
//! - `create_jwt` / `decode_jwt` — access token issuance and validation (HS256)
//! - `generate_refresh_token` — cryptographically secure opaque token
//! - `hash_refresh_token` / `verify_refresh_token` — HMAC-SHA256 for refresh token storage
//! - `constant_time_eq` — timing-safe string comparison
use crate::shared::config::load_env_var::JwtConfig;
use crate::shared::errors::api_errors::ApiError;
use argon2::{
    Argon2,
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        rand_core::{OsRng, RngCore},
    },
};
use chrono::{Duration, Utc};
use hex;
use hmac::{Hmac, Mac};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use subtle::ConstantTimeEq;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

/// JWT claims used in access tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub tv: i32,
}

/// Hash a plaintext password using Argon2id and return the encoded hash string.
pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|_| ApiError::InternalError("Password hashing failed".into()))
}

/// Verify a plaintext password against a stored Argon2id hash.
/// Returns `Ok(true)` on match, `Ok(false)` on mismatch.
pub fn verify_password(hash: &str, password: &str) -> Result<bool, ApiError> {
    let parsed = PasswordHash::new(hash)
        .map_err(|_| ApiError::InternalError("Invalid password hash".into()))?;
    match Argon2::default().verify_password(password.as_bytes(), &parsed) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Constant-time string comparison to prevent timing attacks.
/// Use this whenever comparing tokens or secrets.
pub fn constant_time_eq(a: &str, b: &str) -> bool {
    a.as_bytes().ct_eq(b.as_bytes()).into()
}

/// Create a signed JWT access token for `user_id` including `token_version`.
/// Uses HS256 with the secret from `JwtConfig`.
pub fn create_jwt(user_id: Uuid, token_version: i32, cfg: &JwtConfig) -> Result<String, ApiError> {
    let exp = (Utc::now() + Duration::minutes(cfg.access_exp_minutes)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        tv: token_version,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(cfg.secret.as_ref()),
    )
    .map_err(|_| ApiError::InternalError("Token creation failed".into()))
}

/// Decode and validate a JWT access token. Returns parsed claims or an error.
pub fn decode_jwt(token: &str, cfg: &JwtConfig) -> Result<TokenData<Claims>, ApiError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(cfg.secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| ApiError::BadRequest(format!("Invalid token: {}", e)))
}

/// Generate a cryptographically secure opaque refresh token (128 hex chars = 64 random bytes).
pub fn generate_refresh_token() -> String {
    let mut bytes = [0u8; 64];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Hash a refresh token using HMAC-SHA256 with the JWT secret as the key.
/// The resulting hex string is safe to store in the DB.
///
/// Using HMAC (not Argon2) here is appropriate because refresh tokens are already
/// high-entropy random values — they don't need the slow KDF that passwords need.
pub fn hash_refresh_token(token: &str, cfg: &JwtConfig) -> Result<String, ApiError> {
    let mut mac = HmacSha256::new_from_slice(cfg.secret.as_bytes())
        .map_err(|_| ApiError::InternalError("HMAC key error".into()))?;
    mac.update(token.as_bytes());
    Ok(hex::encode(mac.finalize().into_bytes()))
}

/// Verify a presented refresh token against a stored HMAC-SHA256 hash.
/// Uses constant-time comparison to prevent timing attacks.
pub fn verify_refresh_token(token: &str, stored_hash: &str, cfg: &JwtConfig) -> Result<bool, ApiError> {
    let expected = hash_refresh_token(token, cfg)?;
    Ok(constant_time_eq(&expected, stored_hash))
}

/// Compute refresh token expiry as a Unix timestamp (seconds since epoch).
pub fn refresh_expiry_timestamp(cfg: &JwtConfig) -> i64 {
    (Utc::now() + Duration::days(cfg.refresh_exp_days)).timestamp()
}
