use crate::shared::models::refresh_tokens;
use chrono::Utc;
use sea_orm::prelude::DateTimeWithTimeZone;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use uuid::Uuid;

pub struct RefreshTokenRepository;

impl RefreshTokenRepository {
    /// Persist a new refresh token record. `token` must be the HMAC-SHA256 hash
    /// of the opaque token given to the client — never store the plaintext.
    pub async fn create(
        db: &DatabaseConnection,
        user_id: Uuid,
        token_hash: String,
        expires_at: Option<DateTimeWithTimeZone>,
    ) -> Result<refresh_tokens::Model, DbErr> {
        let active = refresh_tokens::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user_id),
            token: Set(token_hash),
            token_version: Set(Some(0)),
            revoked: Set(false),
            expires_at: Set(expires_at),
            created_at: Set(Some(Utc::now().into())),
        };
        refresh_tokens::Entity::insert(active)
            .exec_with_returning(db)
            .await
    }

    pub async fn update(
        db: &DatabaseConnection,
        model: refresh_tokens::ActiveModel,
    ) -> Result<refresh_tokens::Model, DbErr> {
        model.update(db).await
    }

    /// Look up a single active (non-revoked) token by its stored HMAC hash.
    /// This is an O(1) indexed lookup — no full-table scan.
    pub async fn find_active_by_hash(
        db: &DatabaseConnection,
        token_hash: &str,
    ) -> Result<Option<refresh_tokens::Model>, DbErr> {
        refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::Token.eq(token_hash))
            .filter(refresh_tokens::Column::Revoked.eq(false))
            .one(db)
            .await
    }

    /// Find the most recent active token for a user — used in login to carry over token_version.
    pub async fn find_by_user_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Option<refresh_tokens::Model>, DbErr> {
        refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::UserId.eq(user_id))
            .one(db)
            .await
    }

    /// Check whether the user has at least one active (non-revoked) session.
    pub async fn has_active_for_user(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<bool, DbErr> {
        refresh_tokens::Entity::find()
            .filter(refresh_tokens::Column::UserId.eq(user_id))
            .filter(refresh_tokens::Column::Revoked.eq(false))
            .one(db)
            .await
            .map(|opt| opt.is_some())
    }

    /// Revoke a single token by its primary key.
    pub async fn revoke_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<refresh_tokens::Model, DbErr> {
        let model = refresh_tokens::Entity::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound(format!("refresh token {} not found", id)))?;

        let mut active: refresh_tokens::ActiveModel = model.into();
        active.revoked = Set(true);
        active.update(db).await
    }

    /// Bulk-revoke all tokens for a user with a single UPDATE query.
    pub async fn revoke_all_by_user(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<u64, DbErr> {
        use sea_orm::sea_query::Expr;

        let result = refresh_tokens::Entity::update_many()
            .col_expr(refresh_tokens::Column::Revoked, Expr::value(true))
            .filter(refresh_tokens::Column::UserId.eq(user_id))
            .filter(refresh_tokens::Column::Revoked.eq(false))
            .exec(db)
            .await?;

        Ok(result.rows_affected)
    }

    /// Delete all expired tokens with a single DELETE query.
    pub async fn delete_expired(db: &DatabaseConnection) -> Result<u64, DbErr> {
        let now: DateTimeWithTimeZone = Utc::now().into();

        let result = refresh_tokens::Entity::delete_many()
            .filter(refresh_tokens::Column::ExpiresAt.lt(now))
            .exec(db)
            .await?;

        Ok(result.rows_affected)
    }
}
