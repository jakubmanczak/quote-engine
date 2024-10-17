use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router
};
use sqlite::State;
use tracing::error;

use crate::db::get_conn;

pub fn exported_routes() -> Router {
    Router::new().route("/lines/count", get(get_lines_count))
}

async fn get_lines_count() -> Response {
    let conn = get_conn();
    let query = "SELECT COUNT(*) FROM lines";
    let mut statement = conn.prepare(query).unwrap();
    match statement.next() {
        Ok(State::Row) => {
            let count: i64 = statement.read(0).unwrap();
            return count.to_string().into_response();
        }
        Ok(State::Done) => {
            return (StatusCode::NOT_FOUND, "No lines in database.").into_response()
        }
        Err(e) => {
            error!("Error in GET /lines/count: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }
}
