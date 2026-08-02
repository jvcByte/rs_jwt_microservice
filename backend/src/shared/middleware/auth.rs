use crate::api::refresh_tokens::repository::RefreshTokenRepository;
use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, error, http::header, web};
use futures::future::LocalBoxFuture;
use sea_orm::EntityTrait;
use uuid::Uuid;

use crate::shared::config::app_state::AppState;
use crate::shared::config::load_env_var::JwtConfig;
use crate::shared::utils::auth_utils::decode_jwt;

/// The authenticated user extracted from a valid Bearer JWT.
/// Injected as a handler parameter to require authentication.
///
/// Example:
/// ```ignore
/// async fn my_handler(user: AuthenticatedUser) -> impl Responder { ... }
/// ```
#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl AuthenticatedUser {
    fn err_unauthorized<E: Into<String>>(message: E) -> Error {
        error::ErrorUnauthorized(message.into())
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_owned());

        let app_data = req.app_data::<web::Data<AppState>>().cloned();

        Box::pin(async move {
            // 1. Require Authorization: Bearer <token>
            let auth = auth_header.ok_or_else(|| {
                AuthenticatedUser::err_unauthorized("Missing Authorization header")
            })?;

            let token = auth
                .strip_prefix("Bearer ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .ok_or_else(|| {
                    AuthenticatedUser::err_unauthorized("Invalid or empty Bearer token")
                })?;

            // 2. Decode and validate JWT signature + expiry
            let cfg = JwtConfig::get();
            let token_data = decode_jwt(token, cfg)
                .map_err(|_| AuthenticatedUser::err_unauthorized("Invalid or expired token"))?;

            // 3. Parse user UUID from `sub` claim
            let user_id = Uuid::parse_str(&token_data.claims.sub)
                .map_err(|_| AuthenticatedUser::err_unauthorized("Invalid token subject"))?;

            let state = app_data.ok_or_else(|| {
                AuthenticatedUser::err_unauthorized("Server misconfiguration: missing app state")
            })?;
            let db = &state.db;

            // 4. Load user and check account is active
            let user = entity::users::Entity::find_by_id(user_id)
                .one(db)
                .await
                .map_err(|e| AuthenticatedUser::err_unauthorized(e.to_string()))?
                .ok_or_else(|| AuthenticatedUser::err_unauthorized("User not found"))?;

            if user.is_active == Some(false) {
                return Err(AuthenticatedUser::err_unauthorized("Account is disabled"));
            }

            // 5. Verify token_version claim (tv) against the user's active session.
            //    token_version lives on the refresh_tokens table (associated with the user's
            //    current session). Incrementing it on logout invalidates all access tokens.
            let active_token = RefreshTokenRepository::has_active_for_user(db, user_id)
                .await
                .map_err(|_| {
                    AuthenticatedUser::err_unauthorized("Failed to verify session state")
                })?;

            if !active_token {
                return Err(AuthenticatedUser::err_unauthorized(
                    "No active session — please log in again",
                ));
            }

            // Find the user's latest token record and compare token_version
            let session = RefreshTokenRepository::find_by_user_id(db, user_id)
                .await
                .map_err(|e| AuthenticatedUser::err_unauthorized(e.to_string()))?
                .ok_or_else(|| AuthenticatedUser::err_unauthorized("Session not found"))?;

            let db_tv = session.token_version.unwrap_or(0);
            if token_data.claims.tv != db_tv {
                return Err(AuthenticatedUser::err_unauthorized(
                    "Token has been revoked — please log in again",
                ));
            }

            Ok(AuthenticatedUser {
                id: user.id,
                name: user.name,
                email: user.email,
            })
        })
    }
}
