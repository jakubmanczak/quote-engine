use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::error;

mod auth;
mod db;
mod error;
mod models;
mod permissions;
mod routes;
mod setup;

#[tokio::main]
async fn main() {
    setup::initialise_logging();
    setup::initialise_dotenv();

    let app = Router::new()
        .merge(routes::routes())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

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
