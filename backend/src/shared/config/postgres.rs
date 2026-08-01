use crate::shared::config::load_env_var::EnvVariables;
use crate::shared::utils::config_utils::redact_url_password;
use log::info;
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement,
};
use std::error::Error;
use std::time::Duration;

pub async fn create_database_if_not_exists() -> Result<(), Box<dyn Error + Send + Sync>> {
    let database_url = EnvVariables::get().db_url.clone();

    let (base_url, db_name) = database_url.rsplit_once('/').unwrap();
    let admin_url = format!("{}/postgres", base_url);

    let db = Database::connect(admin_url).await?;

    let stmt = Statement::from_string(
        DbBackend::Postgres,
        format!("SELECT 1 FROM pg_database WHERE datname = '{}'", db_name),
    );

    let exists = db.query_one_raw(stmt).await?.is_some();

    if !exists {
        info!("❎ Database '{}' not found, creating it ❎", db_name);

        let stmt = Statement::from_string(
            DbBackend::Postgres,
            format!("CREATE DATABASE \"{}\"", db_name),
        );

        db.execute_raw(stmt).await?;

        info!("✅ Database '{}' created ✅", db_name);
    }

    Ok(())
}

pub async fn init_db() -> Result<DatabaseConnection, Box<dyn Error + Send + Sync>> {
    let database_url = EnvVariables::get().db_url.clone();

    info!(
        "Connecting to database: {}",
        redact_url_password(&database_url)
    );

    let mut opt = ConnectOptions::new(database_url);
    opt.max_connections(100)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    let db = Database::connect(opt)
        .await
        .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;

    info!("✅ Database connected successfully ✅");
    Ok(db)
}

pub async fn check_connection(db: &DatabaseConnection) -> Result<(), DbErr> {
    db.ping().await
}

pub async fn init_db_with_migrations() -> DatabaseConnection {
    let _ = create_database_if_not_exists().await;

    let db = match init_db().await {
        Ok(db) => db,
        Err(e) => {
            log::error!("Init DB Failed: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = Migrator::up(&db, None).await {
        log::error!("Run Migration Failed: {}", e);
        std::process::exit(1);
    }
    db
}
