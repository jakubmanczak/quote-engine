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
    routing::{delete, get, patch, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlite::State;
use tower_cookies::Cookies;
use tracing::error;
use ulid::Ulid;

pub fn exported_routes() -> Router {
    Router::new()
        .route("/authors", get(get_authors))
        .route("/authors/count", get(get_authors_count))
        .route("/authors/quoted-count", get(get_authors_quoted_count))
        .route("/authors/extended", get(get_extended_authors))
        .route("/authors/:id", get(get_author_by_id))
        .route("/authors/:id/extended", get(get_extended_author_by_id))
        .route(
            "/authors/:id/quote-line-counts",
            get(get_authors_quoteline_counts),
        )
        .route("/authors", post(post_author))
        .route("/authors/:id", patch(patch_author))
        .route("/authors/:id", delete(delete_author))
}

#[derive(Serialize)]
struct ExtendedAuthor {
    id: Ulid,
    name: String,
    obfname: String,
    quotecount: i64,
    linecount: i64,
}

#[derive(Deserialize)]
struct CreateAuthor {
    name: String,
    obfname: String,
}

#[derive(Deserialize)]
struct PatchAuthor {
    name: Option<String>,
    obfname: Option<String>,
}

#[derive(Serialize)]
struct AuthorQuoteLine {
    quotecount: i64,
    linecount: i64,
}

async fn get_authors(headers: HeaderMap, cookies: Cookies) -> Response {
    match authenticate(&headers, cookies) {
        Ok(_) => (),
        Err(e) => return e.log_and_response(),
    };

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

async fn get_extended_authors(headers: HeaderMap, cookies: Cookies) -> Response {
    match authenticate(&headers, cookies) {
        Ok(_) => (),
        Err(e) => return e.log_and_response(),
    };

    let mut authors: Vec<ExtendedAuthor> = Vec::new();
    {
        let conn = get_conn();
        let query = "SELECT * FROM authors";
        let mut statement = conn.prepare(query).unwrap();

        let mut tempauthors: Vec<Author> = Vec::new();

        loop {
            match statement.next() {
                Ok(State::Row) => tempauthors.push(Author {
                    id: ulid::Ulid::from_string(
                        statement.read::<String, _>("id").unwrap().as_str(),
                    )
                    .unwrap(),
                    name: statement.read("name").unwrap(),
                    obfname: statement.read("obfname").unwrap(),
                }),
                Ok(State::Done) => break,
                Err(err) => {
                    error!("error on get extended authors: {}", err);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            }
        }

        for author in tempauthors {
            let quotecount: i64;
            let linecount: i64;

            let query = "SELECT COUNT(*) FROM lines WHERE author = :a";
            let mut statement = conn.prepare(query).unwrap();
            statement
                .bind((":a", author.id.to_string().as_str()))
                .unwrap();

            match statement.next() {
                Ok(_) => {
                    linecount = statement.read(0).unwrap();
                }
                Err(e) => {
                    error!("Error in GET /authors/:id/quote-line-counts: {e}");
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
            }

            let query = "SELECT COUNT(DISTINCT quote) FROM lines WHERE author = :a";
            let mut statement = conn.prepare(query).unwrap();
            statement
                .bind((":a", author.id.to_string().as_str()))
                .unwrap();

            match statement.next() {
                Ok(_) => {
                    quotecount = statement.read(0).unwrap();
                }
                Err(e) => {
                    error!("Error in GET /authors/:id/quote-line-counts: {e}");
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
            }

            authors.push(ExtendedAuthor {
                id: author.id,
                name: author.name,
                obfname: author.obfname,
                quotecount,
                linecount,
            });
        }
    }

    return Json(authors).into_response();
}

async fn get_author_by_id(Path(id): Path<Ulid>, headers: HeaderMap, cookies: Cookies) -> Response {
    match authenticate(&headers, cookies) {
        Ok(_) => (),
        Err(e) => return e.log_and_response(),
    };

    let conn = get_conn();
    let query = "SELECT * FROM authors WHERE id = :id";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((":id", id.to_string().as_str())).unwrap();

    match statement.next() {
        Ok(State::Row) => Json(Author {
            id: id,
            name: statement.read("name").unwrap(),
            obfname: statement.read("obfname").unwrap(),
        })
        .into_response(),
        Ok(State::Done) => (StatusCode::BAD_REQUEST, "No such author found.").into_response(),
        Err(e) => {
            error!("Error in getauthorbyid: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }
}

async fn get_extended_author_by_id(
    Path(id): Path<Ulid>,
    headers: HeaderMap,
    cookies: Cookies,
) -> Response {
    match authenticate(&headers, cookies) {
        Ok(_) => (),
        Err(e) => return e.log_and_response(),
    };

    let name: String;
    let obfname: String;
    let quotecount: i64;
    let linecount: i64;

    {
        let conn = get_conn();
        let query = "SELECT * FROM authors WHERE id = :id";
        let mut statement = conn.prepare(query).unwrap();
        statement.bind((":id", id.to_string().as_str())).unwrap();

        match statement.next() {
            Ok(State::Row) => {
                name = statement.read("name").unwrap();
                obfname = statement.read("obfname").unwrap();
            }
            Ok(State::Done) => {
                return (StatusCode::BAD_REQUEST, "No such author found.").into_response()
            }
            Err(e) => {
                error!("Error in getauthorbyid: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }

        let query = "SELECT COUNT(*) FROM lines WHERE author = :a";
        let mut statement = conn.prepare(query).unwrap();
        statement.bind((":a", id.to_string().as_str())).unwrap();

        match statement.next() {
            Ok(_) => {
                linecount = statement.read(0).unwrap();
            }
            Err(e) => {
                error!("Error in GET /authors/:id/quote-line-counts: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }

        let query = "SELECT COUNT(DISTINCT quote) FROM lines WHERE author = :a";
        let mut statement = conn.prepare(query).unwrap();
        statement.bind((":a", id.to_string().as_str())).unwrap();

        match statement.next() {
            Ok(_) => {
                quotecount = statement.read(0).unwrap();
            }
            Err(e) => {
                error!("Error in GET /authors/:id/quote-line-counts: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    return (Json(ExtendedAuthor {
        id,
        name,
        obfname,
        quotecount,
        linecount,
    }))
    .into_response();
}

async fn post_author(
    headers: HeaderMap,
    cookies: Cookies,
    Json(body): Json<CreateAuthor>,
) -> Response {
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => return e.log_and_response(),
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

async fn patch_author(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<Ulid>,
    Json(body): Json<PatchAuthor>,
) -> Response {
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => return e.log_and_response(),
    };

    match UserPermission::check_permission(&UserPermission::ModifyAuthorsNames, &actor.perms) {
        true => (),
        false => return StatusCode::FORBIDDEN.into_response(),
    };

    let subject = {
        let conn = get_conn();
        let query = "SELECT * FROM authors WHERE id = :id";
        let mut st = conn.prepare(query).unwrap();
        st.bind((":id", id.to_string().as_str())).unwrap();

        match st.next() {
            Ok(State::Row) => Author {
                id: id,
                name: st.read("name").unwrap(),
                obfname: st.read("obfname").unwrap(),
            },
            Ok(State::Done) => {
                return (StatusCode::BAD_REQUEST, "Author not found").into_response()
            }
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    };

    let n = match body.name {
        Some(name) => name,
        None => subject.name.clone(),
    };
    let o = match body.obfname {
        Some(obfname) => obfname,
        None => subject.obfname.clone(),
    };
    {
        let conn = get_conn();
        let query = "UPDATE authors SET name = :n, obfname = :o WHERE id = :id";
        let mut st = conn.prepare(query).unwrap();
        st.bind((":id", id.to_string().as_str())).unwrap();
        st.bind((":n", n.as_str())).unwrap();
        st.bind((":o", o.as_str())).unwrap();

        match st.next() {
            Ok(_) => (),
            Err(e) => {
                error!("could not update author: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    let new = Author {
        id: id,
        name: n,
        obfname: o,
    };
    push_log(LogEntry {
        id: Ulid::new(),
        timestamp: Utc::now().timestamp(),
        actor: actor.id,
        action: LogEvent::AuthorUpdated {
            old: subject,
            new: new.clone(),
        },
        subject: id,
    });

    return Json(new).into_response();
}

async fn delete_author(headers: HeaderMap, cookies: Cookies, Path(id): Path<Ulid>) -> Response {
    // TODO: INTRODUCE CHECKING FOR LINES ASSOCIATED WITH AUTHOR
    // AND DISALLOW DELETION IF ORPHANED LINES WOULD BE CREATED
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => return e.log_and_response(),
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
            error!("Error in GET /authors/count: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }
}

async fn get_authors_quoted_count() -> Response {
    let conn = get_conn();
    let query = "SELECT COUNT(DISTINCT author) FROM lines";
    let mut statement = conn.prepare(query).unwrap();
    match statement.next() {
        Ok(_) => {
            let count: i64 = statement.read(0).unwrap();
            return count.to_string().into_response();
        }
        Err(e) => {
            error!("Error in GET /users/quoted-count: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }
}

async fn get_authors_quoteline_counts(Path(id): Path<Ulid>) -> Response {
    let quotes: i64;
    let lines: i64;

    {
        let conn = get_conn();
        let query = "SELECT COUNT(*) FROM lines WHERE author = :a";
        let mut statement = conn.prepare(query).unwrap();
        statement.bind((":a", id.to_string().as_str())).unwrap();

        match statement.next() {
            Ok(_) => {
                lines = statement.read(0).unwrap();
            }
            Err(e) => {
                error!("Error in GET /authors/:id/quote-line-counts: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }
    {
        let conn = get_conn();
        let query = "SELECT COUNT(DISTINCT quote) FROM lines WHERE author = :a";
        let mut statement = conn.prepare(query).unwrap();
        statement.bind((":a", id.to_string().as_str())).unwrap();

        match statement.next() {
            Ok(_) => {
                quotes = statement.read(0).unwrap();
            }
            Err(e) => {
                error!("Error in GET /authors/:id/quote-line-counts: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    return Json(AuthorQuoteLine {
        quotecount: quotes,
        linecount: lines,
    })
    .into_response();
}
