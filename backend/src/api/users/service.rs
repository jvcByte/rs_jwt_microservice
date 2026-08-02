use crate::api::auth::repository::RefreshTokenRepository;
use crate::api::users::dto::{CreateUser, UpdateUser, UserResponse};
use crate::api::users::repository::UserRepository;
use crate::shared::config::load_env_var::JwtConfig;
use crate::shared::errors::api_errors::ApiError;
use crate::shared::models::refresh_tokens::ActiveModel as RefreshTokenActiveModel;
use crate::shared::models::users::ActiveModel;
use crate::shared::utils::auth_utils::{create_jwt, hash_password, verify_password};
use chrono::Utc;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub struct UserService;

impl UserService {
    /// Register a new user with a hashed password.
    pub async fn register_user(
        db: &DatabaseConnection,
        input: CreateUser,
        password: String,
    ) -> Result<Uuid, ApiError> {
        if input.name.trim().is_empty() {
            return Err(ApiError::BadRequest("Name cannot be empty".into()));
        }
        if input.email.trim().is_empty() {
            return Err(ApiError::BadRequest("Email cannot be empty".into()));
        }
        // Basic email format validation
        if !is_valid_email(&input.email) {
            return Err(ApiError::BadRequest("Invalid email format".into()));
        }
        if password.len() < 8 {
            return Err(ApiError::BadRequest(
                "Password must be at least 8 characters".into(),
            ));
        }

        // Check email uniqueness
        if UserRepository::find_by_email(db, &input.email)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?
            .is_some()
        {
            return Err(ApiError::Conflict("Email already exists".into()));
        }

        let id = Uuid::new_v4();
        let active = ActiveModel {
            id: Set(id),
            name: Set(input.name),
            email: Set(input.email),
            password_hash: Set(hash_password(&password)?),
            is_active: Set(Some(true)),
            created_at: Set(Some(Utc::now().into())),
            ..Default::default()
        };

        UserRepository::insert(db, active)
            .await
            .map_err(|e| ApiError::InternalError(format!("DB insert failed: {}", e)))?;

        Ok(id)
    }

    /// Authenticate a user and return a signed JWT.
    /// The token_version is read from the user's latest refresh token record.
    /// If no refresh token exists yet (first login), tv defaults to 0.
    pub async fn login(
        db: &DatabaseConnection,
        email: &str,
        password: &str,
    ) -> Result<String, ApiError> {
        if email.trim().is_empty() || password.is_empty() {
            return Err(ApiError::BadRequest(
                "Email and password must be provided".into(),
            ));
        }

        let user = UserRepository::find_by_email(db, email)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?
            // Use a generic message to avoid user enumeration
            .ok_or_else(|| ApiError::BadRequest("Invalid email or password".into()))?;

        if user.is_active == Some(false) {
            return Err(ApiError::BadRequest("Account is disabled".into()));
        }

        if !verify_password(&user.password_hash, password)? {
            return Err(ApiError::BadRequest("Invalid email or password".into()));
        }

        // Read current token_version from refresh_tokens (defaults to 0 if none yet)
        let tv = RefreshTokenRepository::find_by_user_id(db, user.id)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?
            .and_then(|rt| rt.token_version)
            .unwrap_or(0);

        let token = create_jwt(user.id, tv, JwtConfig::get())?;

        // Update last_login timestamp
        let active = ActiveModel {
            id: Set(user.id),
            last_login: Set(Some(Utc::now().into())),
            ..Default::default()
        };
        UserRepository::update(db, active)
            .await
            .map_err(|e| ApiError::InternalError(format!("DB update failed: {}", e)))?;

        Ok(token)
    }

    pub async fn list_users(db: &DatabaseConnection) -> Result<Vec<UserResponse>, ApiError> {
        UserRepository::find_all(db)
            .await
            .map_err(|_| ApiError::InternalError("DB error".into()))
            .map(|users| {
                users
                    .into_iter()
                    .map(|m| UserResponse { id: m.id, name: m.name, email: m.email })
                    .collect()
            })
    }

    pub async fn get_user(db: &DatabaseConnection, id: Uuid) -> Result<UserResponse, ApiError> {
        if id == Uuid::nil() {
            return Err(ApiError::BadRequest("Invalid UUID".into()));
        }
        UserRepository::find_by_id(db, id)
            .await
            .map_err(|_| ApiError::InternalError("DB error".into()))?
            .map(|u| UserResponse { id: u.id, name: u.name, email: u.email })
            .ok_or_else(|| ApiError::NotFound(format!("User {} not found", id)))
    }

    pub async fn update_user(
        db: &DatabaseConnection,
        id: Uuid,
        input: UpdateUser,
    ) -> Result<UserResponse, ApiError> {
        let existing = UserRepository::find_by_id(db, id)
            .await
            .map_err(|_| ApiError::InternalError("DB error".into()))?
            .ok_or_else(|| ApiError::NotFound(format!("User {} not found", id)))?;

        let mut active: ActiveModel = existing.into();

        if let Some(name) = input.name {
            if name.trim().is_empty() {
                return Err(ApiError::BadRequest("Name cannot be empty".into()));
            }
            active.name = Set(name);
        }

        if let Some(email) = input.email {
            if email.trim().is_empty() {
                return Err(ApiError::BadRequest("Email cannot be empty".into()));
            }
            if !is_valid_email(&email) {
                return Err(ApiError::BadRequest("Invalid email format".into()));
            }
            if UserRepository::find_by_email(db, &email)
                .await
                .map_err(|_| ApiError::InternalError("DB error".into()))?
                .filter(|u| u.id != id)
                .is_some()
            {
                return Err(ApiError::Conflict("Email already exists".into()));
            }
            active.email = Set(email);
        }

        active.updated_at = Set(Some(Utc::now().into()));

        let updated = UserRepository::update(db, active)
            .await
            .map_err(|_| ApiError::InternalError("DB update failed".into()))?;

        Ok(UserResponse { id: updated.id, name: updated.name, email: updated.email })
    }

    pub async fn delete_user(db: &DatabaseConnection, id: Uuid) -> Result<(), ApiError> {
        if id == Uuid::nil() {
            return Err(ApiError::BadRequest("Invalid UUID".into()));
        }
        let rows = UserRepository::delete(db, id)
            .await
            .map_err(|_| ApiError::InternalError("DB delete failed".into()))?;

        if rows == 0 {
            return Err(ApiError::NotFound(format!("User {} not found", id)));
        }
        Ok(())
    }

    /// Increment the token_version on the user's active refresh token record to
    /// invalidate all outstanding access tokens for this user.
    pub async fn increment_token_version(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<(), ApiError> {
        // Find all non-revoked tokens and bump their version
        use crate::shared::models::refresh_tokens;
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

        let tokens = refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::UserId.eq(user_id))
            .filter(refresh_tokens::Column::Revoked.eq(false))
            .all(db)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?;

        for token in tokens {
            let new_version = token.token_version.unwrap_or(0) + 1;
            let mut active: RefreshTokenActiveModel = token.into();
            active.token_version = Set(Some(new_version));
            RefreshTokenRepository::update(db, active)
                .await
                .map_err(|e| ApiError::InternalError(e.to_string()))?;
        }

        Ok(())
    }
}

/// Basic email format check: must contain exactly one '@' with non-empty local and domain parts,
/// and the domain must contain a '.'.
fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.splitn(2, '@').collect();
    if parts.len() != 2 {
        return false;
    }
    let local = parts[0];
    let domain = parts[1];
    !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}
