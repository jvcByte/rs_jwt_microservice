use crate::api::home::routes::home_routes;
use crate::api::routes::routes;
use crate::shared::config::app_state::AppState;
use crate::shared::config::cors::cors;
use crate::shared::config::load_env_var::EnvVariables;
use actix_web::middleware::{Logger, NormalizePath};
use actix_web::{App, HttpServer, web};
use sea_orm::DatabaseConnection;

pub async fn run(db: DatabaseConnection) -> std::io::Result<()> {
    let state = web::Data::new(AppState::new(db));

    let address = EnvVariables::get().address.clone();
    let port = EnvVariables::get().port.clone();
    let base_url = format!("{}:{}", address, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .wrap(cors())
            .app_data(state.clone())
            .configure(home_routes)
            .configure(routes)
    })
    .bind(base_url)?
    .run()
    .await
}
