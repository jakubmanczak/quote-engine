use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    Router,
};
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

use crate::{routes, setup, users::defaultadmin::check_lack_of_account};

pub async fn init() -> Router {
    let pool = setup::init_database_pool().await;
    check_lack_of_account(&pool).await;

    let origins = ["http://localhost:3000"];
    Router::new()
        .merge(routes::all())
        .with_state(pool)
        .layer(
            CorsLayer::new()
                .allow_origin(origins.map(|s| s.parse().unwrap()))
                .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                .allow_headers([AUTHORIZATION, CONTENT_TYPE])
                .allow_credentials(true),
        )
        .layer(CookieManagerLayer::new())
}
