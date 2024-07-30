use crate::{
    auth::{get_auth_from_header, validate::validate_basic_auth, AuthType},
    db::{
        get_conn,
        users::{get_user_data, GetUserDataInput},
    },
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
    let auth = match get_auth_from_header(&headers) {
        Some(auth) => match auth {
            AuthType::Basic(auth) => match validate_basic_auth(&auth) {
                true => auth,
                false => return StatusCode::UNAUTHORIZED.into_response(),
            },
            _ => return StatusCode::UNAUTHORIZED.into_response(),
        },
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let actor = match get_user_data(GetUserDataInput::Name(auth.user)) {
        Ok(user) => user,
        Err(e) => {
            error!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
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
