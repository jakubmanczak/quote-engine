use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use sqlite::State;
use tracing::error;

use crate::db::get_conn;

pub fn exported_routes() -> Router {
    Router::new().route("/quotes/count", get(get_quotes_count))
}

async fn get_quotes_count() -> Response {
    let conn = get_conn();
    let query = "SELECT COUNT(*) FROM quotes";
    let mut statement = conn.prepare(query).unwrap();
    match statement.next() {
        Ok(State::Row) => {
            let count: i64 = statement.read(0).unwrap();
            return count.to_string().into_response();
        }
        Ok(State::Done) => {
            return (StatusCode::NOT_FOUND, "No quotes in database.").into_response()
        }
        Err(e) => {
            error!("Error in GET /users/count: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }
}
