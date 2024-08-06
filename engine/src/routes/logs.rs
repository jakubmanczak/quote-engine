use crate::{
    auth::authenticate_via_basicauth,
    db::get_conn,
    models::{Log, Pagination},
    permissions::UserPermission,
};
use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use sqlite::State;
use tracing::error;

pub fn exported_routes() -> Router {
    Router::new().route("/logs", get(logs_route))
}

async fn logs_route(headers: HeaderMap, Query(p): Query<Pagination>) -> Response {
    let actor = match authenticate_via_basicauth(&headers) {
        Ok(user) => user,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };

    match UserPermission::check_permission(&UserPermission::InspectLogs, &actor.perms) {
        true => (),
        false => return StatusCode::FORBIDDEN.into_response(),
    }

    let mut logs: Vec<Log> = Vec::new();
    let limit = i64::from(u32::from(p.limit));
    let offset = i64::from(u32::from(p.limit) * (u32::from(p.page) - 1));
    let q = "SELECT * FROM logs ORDER BY id DESC LIMIT :limit OFFSET :offset";
    {
        let conn = get_conn();
        let mut statement = conn.prepare(q).unwrap();
        statement.bind((":limit", limit)).unwrap();
        statement.bind((":offset", offset)).unwrap();

        loop {
            match statement.next() {
                Ok(State::Row) => logs.push(Log {
                    id: statement.read("id").unwrap(),
                    timestamp: match u64::try_from(statement.read::<i64, _>("timestamp").unwrap()) {
                        Ok(v) => v,
                        Err(e) => {
                            let res = format!("Could not convert db i64 to timestamp u64: {e}");
                            return (StatusCode::INTERNAL_SERVER_ERROR, res).into_response();
                        }
                    },
                    content: statement.read("content").unwrap(),
                }),
                Ok(State::Done) => match logs.is_empty() {
                    true => {
                        return (StatusCode::NOT_FOUND, "No logs found for query.").into_response()
                    }
                    false => return Json(logs).into_response(),
                },
                Err(e) => {
                    error!("Error in GET /logs: {e}");
                    return (StatusCode::INTERNAL_SERVER_ERROR, format!("{e}")).into_response();
                }
            }
        }
    }
}
