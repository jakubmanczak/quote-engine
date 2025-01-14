use axum::{
    extract::{
        ws::{Message, Utf8Bytes, WebSocket},
        State, WebSocketUpgrade,
    },
    http::HeaderMap,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use tower_cookies::Cookies;
use tracing::error;

use crate::{state::SharedState, user::User};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/health", get(health))
        .route("/health/ws", get(health_ws_upgrade))
}

async fn health(
    headers: HeaderMap,
    cookies: Cookies,
    State(state): State<SharedState>,
) -> Response {
    // infosec: only show system health to actual users
    // match User::authenticate(&headers, cookies, &state.dbpool).await {
    //     Ok(_) => (),
    //     Err(e) => return e.respond(),
    // };

    let sysinfo = state.sysinfo.read().await;
    Json(&*sysinfo).into_response()
}

async fn health_ws_upgrade(
    headers: HeaderMap,
    cookies: Cookies,
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> Response {
    // infosec: only show system health to actual users
    // match User::authenticate(&headers, cookies, &state.dbpool).await {
    //     Ok(_) => (),
    //     Err(e) => return e.respond(),
    // }
    ws.on_upgrade(|ws| async { health_ws_stream(state, ws).await })
}

async fn health_ws_stream(state: SharedState, mut ws: WebSocket) {
    let mut rx = state.syscast.subscribe();
    // TODO: validate client still exists every so often

    while let Ok(msg) = rx.recv().await {
        let msg = match serde_json::to_string(&msg) {
            Ok(msg) => Utf8Bytes::from(msg),
            Err(e) => {
                error!("error serializing message: {}", e);
                continue;
            }
        };
        match ws.send(Message::Text(msg)).await {
            Ok(_) => (),
            Err(e) => {
                error!("error sending message: {}", e);
                continue;
            }
        }
    }
}
