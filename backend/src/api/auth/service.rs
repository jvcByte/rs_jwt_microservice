use crate::api::auth::repository::RefreshTokenRepository;
use crate::shared::config::load_env_var::JwtConfig;
use crate::shared::errors::api_errors::ApiError;
use crate::shared::utils::auth_utils::{
    create_jwt, generate_refresh_token, hash_refresh_token, refresh_expiry_timestamp,
    verify_refresh_token,
};
use chrono::Utc;
use sea_orm::DatabaseConnection;
use sea_orm::prelude::DateTimeWithTimeZone;
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    /// Create a new opaque refresh token, persist its HMAC-SHA256 hash, and return the plain token.
    pub async fn create_refresh_for_user(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<String, ApiError> {
        let cfg = JwtConfig::get();
        let plain = generate_refresh_token();
        let token_hash = hash_refresh_token(&plain, cfg)?;

        let expires_at = Some(DateTimeWithTimeZone::from(
            chrono::DateTime::from_timestamp(refresh_expiry_timestamp(cfg), 0)
                .ok_or_else(|| ApiError::InternalError("Failed to compute expiry".into()))?,
        ));

        RefreshTokenRepository::create(db, user_id, token_hash, expires_at)
            .await
            .map_err(|e| ApiError::InternalError(format!("DB error storing refresh token: {}", e)))?;

        Ok(plain)
    }

    /// Verify an incoming refresh token (by HMAC hash lookup), rotate it, and return
    /// a new access token + new plain refresh token.
    pub async fn verify_and_rotate_refresh(
        db: &DatabaseConnection,
        incoming_plain: &str,
    ) -> Result<(String, String), ApiError> {
        let cfg = JwtConfig::get();

        // Hash the incoming token and look it up directly — O(1) indexed query.
        let incoming_hash = hash_refresh_token(incoming_plain, cfg)?;

        let record = RefreshTokenRepository::find_active_by_hash(db, &incoming_hash)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?
            .ok_or_else(|| ApiError::NotFound("Invalid refresh token".into()))?;

        // Constant-time verification as a second layer (defense in depth)
        if !verify_refresh_token(incoming_plain, &record.token, cfg)? {
            return Err(ApiError::NotFound("Invalid refresh token".into()));
        }

        if record.revoked {
            return Err(ApiError::NotFound("Refresh token revoked".into()));
        }

        if let Some(exp) = record.expires_at {
            if exp.timestamp() < Utc::now().timestamp() {
                return Err(ApiError::NotFound("Refresh token expired".into()));
            }
        }

        let tv = record.token_version.unwrap_or(0);
        let access_token = create_jwt(record.user_id, tv, cfg)?;

        // Issue new refresh token and revoke old one (rotation)
        let new_plain = generate_refresh_token();
        let new_hash = hash_refresh_token(&new_plain, cfg)?;
        let new_expires_at = Some(DateTimeWithTimeZone::from(
            chrono::DateTime::from_timestamp(refresh_expiry_timestamp(cfg), 0)
                .ok_or_else(|| ApiError::InternalError("Failed to compute expiry".into()))?,
        ));

        RefreshTokenRepository::create(db, record.user_id, new_hash, new_expires_at)
            .await
            .map_err(|_| ApiError::InternalError("Failed to store new refresh token".into()))?;

        RefreshTokenRepository::revoke_by_id(db, record.id)
            .await
            .map_err(|_| ApiError::InternalError("Failed to revoke old refresh token".into()))?;

        Ok((access_token, new_plain))
    }

    /// Revoke a specific refresh token presented by the client.
    /// Returns the user_id so the caller can also bump token_version.
    pub async fn revoke_refresh_token(
        db: &DatabaseConnection,
        incoming_plain: &str,
    ) -> Result<Uuid, ApiError> {
        let cfg = JwtConfig::get();
        let incoming_hash = hash_refresh_token(incoming_plain, cfg)?;

        let record = RefreshTokenRepository::find_active_by_hash(db, &incoming_hash)
            .await
            .map_err(|_| ApiError::InternalError("DB error".into()))?
            .ok_or_else(|| ApiError::NotFound("Invalid refresh token".into()))?;

        // Constant-time check as second layer
        if !verify_refresh_token(incoming_plain, &record.token, cfg)? {
            return Err(ApiError::NotFound("Invalid refresh token".into()));
        }

        RefreshTokenRepository::revoke_by_id(db, record.id)
            .await
            .map_err(|_| ApiError::InternalError("Failed to revoke refresh token".into()))?;

        Ok(record.user_id)
    }

    /// Revoke all refresh tokens for the given user. Returns count revoked.
    pub async fn revoke_all_for_user(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<u64, ApiError> {
        RefreshTokenRepository::revoke_all_by_user(db, user_id)
            .await
            .map_err(|e| ApiError::InternalError(format!("DB error revoking tokens: {}", e)))
    }

    /// Delete expired refresh tokens. Returns count deleted.
    pub async fn cleanup_expired(db: &DatabaseConnection) -> Result<u64, ApiError> {
        RefreshTokenRepository::delete_expired(db)
            .await
            .map_err(|e| ApiError::InternalError(format!("DB error cleaning tokens: {}", e)))
    }
}
