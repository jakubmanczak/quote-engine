use crate::{
    auth::authenticate,
    db::get_conn,
    logs::{push_log, LogEntry, LogEvent},
    models::Author,
    permissions::UserPermission,
};
use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use chrono::Utc;
use serde::Deserialize;
use sqlite::State;
use tower_cookies::Cookies;
use tracing::error;
use ulid::Ulid;

pub fn exported_routes() -> Router {
    Router::new()
        .route("/authors", get(get_authors))
        .route("/authors/count", get(get_authors_count))
        .route("/authors", post(post_author))
        .route("/authors/:id", delete(delete_author))
}

async fn get_authors() -> Response {
    let conn = get_conn();
    let query = "SELECT * FROM authors";
    let mut statement = conn.prepare(query).unwrap();

    let mut authors: Vec<Author> = Vec::new();
    loop {
        match statement.next() {
            Ok(State::Row) => authors.push(Author {
                id: ulid::Ulid::from_string(statement.read::<String, _>("id").unwrap().as_str())
                    .unwrap(),
                name: statement.read("name").unwrap(),
                obfname: statement.read("obfname").unwrap(),
            }),
            Ok(State::Done) => return (StatusCode::OK, Json(authors)).into_response(),
            Err(err) => {
                error!("error on get authors: {}", err);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    }
}

#[derive(Deserialize)]
struct CreateAuthor {
    name: String,
    obfname: String,
}

async fn post_author(
    headers: HeaderMap,
    cookies: Cookies,
    Json(body): Json<CreateAuthor>,
) -> Response {
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };

    match UserPermission::check_permission(&UserPermission::CreateAuthors, &actor.perms) {
        false => return StatusCode::FORBIDDEN.into_response(),
        true => (),
    };

    let author = Author {
        id: Ulid::new(),
        name: body.name,
        obfname: body.obfname,
    };

    {
        let conn = get_conn();
        let query = "INSERT INTO authors VALUES (:id, :name, :obfname)";
        let mut st = conn.prepare(query).unwrap();
        st.bind((":id", author.id.to_string().as_str())).unwrap();
        st.bind((":name", author.name.as_str())).unwrap();
        st.bind((":obfname", author.obfname.as_str())).unwrap();

        match st.next() {
            Ok(_) => (),
            Err(e) => {
                error!("Could not create author: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    push_log(LogEntry {
        id: Ulid::new(),
        timestamp: Utc::now().timestamp(),
        actor: actor.id,
        action: LogEvent::AuthorCreated(author.clone()),
        subject: author.id,
    });

    return Json(author).into_response();
}

async fn delete_author(headers: HeaderMap, cookies: Cookies, Path(id): Path<Ulid>) -> Response {
    // TODO: INTRODUCE CHECKING FOR LINES ASSOCIATED WITH AUTHOR
    // AND DISALLOW DELETION IF ORPHANED LINES WOULD BE CREATED
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };

    match UserPermission::check_permission(&UserPermission::DeleteAuthors, &actor.perms) {
        false => return StatusCode::FORBIDDEN.into_response(),
        true => (),
    };

    let author = {
        let conn = get_conn();
        let query = "SELECT * FROM authors WHERE id = :id";
        let mut st = conn.prepare(query).unwrap();
        st.bind((":id", id.to_string().as_str())).unwrap();

        match st.next() {
            Ok(State::Row) => Author {
                id: id.clone(),
                name: st.read("name").unwrap(),
                obfname: st.read("obfname").unwrap(),
            },
            Ok(State::Done) => {
                return (StatusCode::BAD_REQUEST, "No author with such ID.").into_response()
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    match e.message {
                        Some(str) => str,
                        None => "Unknown error".to_owned(),
                    },
                )
                    .into_response()
            }
        }
    };

    {
        let conn = get_conn();
        let query = "DELETE FROM authors WHERE id = :id";
        let mut st = conn.prepare(query).unwrap();
        st.bind((":id", id.to_string().as_str())).unwrap();

        match st.next() {
            Ok(_) => (),
            Err(e) => {
                error!("Could not delete user: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    push_log(LogEntry {
        id: Ulid::new(),
        timestamp: Utc::now().timestamp(),
        actor: actor.id,
        subject: id,
        action: LogEvent::AuthorDeleted(author),
    });
    return StatusCode::NO_CONTENT.into_response();
}

async fn get_authors_count() -> Response {
    let conn = get_conn();
    let query = "SELECT COUNT(*) FROM authors";
    let mut statement = conn.prepare(query).unwrap();
    match statement.next() {
        Ok(_) => {
            let count: i64 = statement.read(0).unwrap();
            return count.to_string().into_response();
        }
        Err(e) => {
            error!("Error in GET /users/count: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }
}
