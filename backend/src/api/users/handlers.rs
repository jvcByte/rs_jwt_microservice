use super::service::UserService;
use crate::api::users::dto::UpdateUser;
use crate::shared::config::app_state::AppState;
use crate::shared::errors::api_errors::ApiError;
use crate::shared::middleware::auth::AuthenticatedUser;
use actix_web::{HttpResponse, Result, web};
use uuid::Uuid;

/// List all users. Requires authentication.
///
/// NOTE: In production you likely want to restrict this to admin roles.
/// Add a role check here before shipping.
pub async fn list_users(
    _user: AuthenticatedUser,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let users = UserService::list_users(&state.db).await?;
    Ok(HttpResponse::Ok().json(users))
}

/// Get a user by ID. Any authenticated user can view any profile.
pub async fn get_user(
    _user: AuthenticatedUser,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let user = UserService::get_user(&state.db, path.into_inner()).await?;
    Ok(HttpResponse::Ok().json(user))
}

/// Update a user. Users can only update their own account.
pub async fn update_user(
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    body: web::Json<UpdateUser>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();

    if user.id != id {
        return Err(ApiError::Forbidden("You can only update your own account".into()));
    }

    let updated = UserService::update_user(&state.db, id, body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(updated))
}

/// Delete a user. Users can only delete their own account.
pub async fn delete_user(
    user: AuthenticatedUser,
    path: web::Path<Uuid>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();

    if user.id != id {
        return Err(ApiError::Forbidden("You can only delete your own account".into()));
    }

    UserService::delete_user(&state.db, id).await?;
    Ok(HttpResponse::NoContent().finish())
}
