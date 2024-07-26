use axum::Router;
use tokio::net::TcpListener;
use tracing::error;

mod auth;
mod db;
mod error;
mod routes;
mod setup;

#[tokio::main]
async fn main() {
    setup::initialise_logging();

    let app = Router::new().merge(routes::routes());
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

    axum::serve(listener, app).await.unwrap();
}
