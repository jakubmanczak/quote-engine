use tracing::{error, info};

mod database;
mod omnierror;
mod router;
mod setup;
mod state;
mod user;

#[tokio::main]
async fn main() {
    setup::init_tracing_and_dotenv();
    setup::verify_required_env_vars();

    let state = state::init().await;
    let router = router::init(state.clone());

    let listener = setup::init_listener().await;

    setup::signal_readiness();
    match axum::serve(listener, router).await {
        Ok(_) => info!("Server stopped."),
        Err(e) => error!("Server error: {e}"),
    };
}
