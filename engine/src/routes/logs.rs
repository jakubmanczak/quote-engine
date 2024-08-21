use crate::{
    auth::authenticate, db::get_conn, logs::LogEntry, models::Pagination,
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
use tower_cookies::Cookies;
use tracing::error;
use ulid::Ulid;

pub fn exported_routes() -> Router {
    Router::new().route("/logs", get(logs_route))
}

async fn logs_route(headers: HeaderMap, Query(p): Query<Pagination>, cookies: Cookies) -> Response {
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };

    match UserPermission::check_permission(&UserPermission::InspectLogs, &actor.perms) {
        true => (),
        false => return StatusCode::FORBIDDEN.into_response(),
    }

    let mut logs: Vec<LogEntry> = Vec::new();
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
                Ok(State::Row) => {
                    let details: String = statement.read("details").unwrap();
                    logs.push(LogEntry {
                        id: Ulid::from_string(statement.read::<String, _>("id").unwrap().as_str())
                            .unwrap(),
                        timestamp: statement.read("timestamp").unwrap(),
                        actor: Ulid::from_string(
                            statement.read::<String, _>("actor").unwrap().as_str(),
                        )
                        .unwrap(),
                        subject: Ulid::from_string(
                            statement.read::<String, _>("subject").unwrap().as_str(),
                        )
                        .unwrap(),
                        action: serde_json::from_str(details.as_str()).unwrap(),
                    });
                }
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
