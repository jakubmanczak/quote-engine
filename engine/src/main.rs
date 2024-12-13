mod auth;
mod authors;
mod error;
mod router;
mod routes;
mod setup;
mod users;

#[tokio::main]
async fn main() {
    setup::initialise_logging();
    setup::initialise_dotenv();
    setup::verify_dburl_presence();
    setup::verify_secret_presence();

    let app = router::init().await;
    let listener = setup::initialise_listener().await;

    axum::serve(listener, app).await.unwrap();
}
