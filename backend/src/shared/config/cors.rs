use actix_cors::Cors;
use actix_web::http::header;

/**
 * The `cors()` function sets up the Cross-Origin Resource Sharing (CORS) policy for the application.
 * It allows requests using the `allowed_origin()` method.
 */
pub fn cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:1420")
        .allowed_origin("http://localhost:3000")
        .allowed_origin("https://tauri.localhost")
        .allowed_origin("http://tauri.localhost")
        .allowed_origin("tauri://localhost")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
        ])
        .expose_headers(vec![header::CONTENT_TYPE])
        .max_age(3600)
        .supports_credentials()
}
