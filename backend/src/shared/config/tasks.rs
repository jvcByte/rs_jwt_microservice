use crate::{api::refresh_tokens::repository::RefreshTokenRepository, log_error, log_info};
use sea_orm::DatabaseConnection;
use tokio::time::{Duration, interval};

pub fn cleanup_expired_refresh_tokens(db: DatabaseConnection) {
    let mut interval = interval(Duration::from_secs(6 * 3600));
    tokio::spawn(async move {
        loop {
            interval.tick().await;
            match RefreshTokenRepository::delete_expired(&db).await {
                Ok(n) => log_info!("Cleanup: {} expired refresh token deleted", n),
                Err(e) => log_error!("Cleanup: Failed to delete expired refresh token: {}", e),
            }
        }
    });
}
