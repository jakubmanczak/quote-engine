use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    Router,
};
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tracing::error;

mod auth;
mod db;
mod error;
mod logs;
mod models;
mod permissions;
mod routes;
mod setup;

#[tokio::main]
async fn main() {
    setup::initialise_logging();
    setup::initialise_dotenv();
    setup::verify_secret_presence();

    let origins = ["http://localhost:3000"];
    let app = Router::new()
        .merge(routes::routes())
        .layer(
            CorsLayer::new()
                .allow_origin(origins.map(|s| s.parse().unwrap()))
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                .allow_credentials(true),
        )
        .layer(CookieManagerLayer::new());

    let addr = setup::get_socket_addr();
    let listener = match TcpListener::bind(&addr).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("error creating a listener: {e}");
            panic!();
        }
    };
    setup::report_listener_socket_addr(&listener);

    db::execute_migration_queries();
    db::check_for_lack_of_account();

    axum::serve(listener, app).await.unwrap();
}
