mod api;
mod shared;
use crate::shared::config::{database, http_server, load_env_var, logger, tasks};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::setup_logger();
    load_env_var::init_vars();
    let db = database::init_db_with_migrations().await;

    // Background tasks
    tasks::cleanup_expired_refresh_tokens(db.clone());

    http_server::run(db).await
}
