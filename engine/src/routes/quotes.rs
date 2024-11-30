use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::{Days, Months, Utc};
use serde::Deserialize;
use sqlite::State;
use std::collections::HashMap;
use tower_cookies::Cookies;
use tracing::error;
use ulid::Ulid;

use crate::{
    auth::authenticate,
    db::{self, get_conn},
    models::{Author, Line, Quote},
    permissions::UserPermission,
};

pub fn exported_routes() -> Router {
    Router::new()
        .route("/quotes/count", get(get_quotes_count))
        .route("/quotes/count/thisweek", get(get_quotes_thisweek_count))
        .route("/quotes/count/thismonth", get(get_quotes_thismonth_count))
        .route("/quotes", get(get_quotes))
        .route("/quotes", post(add_quote))
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

async fn get_quotes_thisweek_count() -> Response {
    let conn = get_conn();
    let query = "SELECT COUNT(*) FROM quotes WHERE timestamp > :ts";
    let ts = match Utc::now().checked_sub_days(Days::new(7)) {
        Some(dt) => dt.timestamp(),
        None => return (StatusCode::INTERNAL_SERVER_ERROR, "dt minus 7d invalid").into_response(),
    };
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((":ts", ts)).unwrap();

    match statement.next() {
        Ok(_) => {
            let c: i64 = statement.read(0).unwrap();
            return c.to_string().into_response();
        }
        Err(e) => {
            error!("Error in GET /users/count/thisweek: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }
}

async fn get_quotes_thismonth_count() -> Response {
    let conn = get_conn();
    let query = "SELECT COUNT(*) FROM quotes WHERE timestamp > :ts";
    let ts = match Utc::now().checked_sub_months(Months::new(1)) {
        Some(dt) => dt.timestamp(),
        None => return (StatusCode::INTERNAL_SERVER_ERROR, "dt minus 1mo invalid").into_response(),
    };
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((":ts", ts)).unwrap();

    match statement.next() {
        Ok(_) => {
            let c: i64 = statement.read(0).unwrap();
            return c.to_string().into_response();
        }
        Err(e) => {
            error!("Error in GET /users/count/thismonth: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }
}

async fn get_quotes(headers: HeaderMap, cookies: Cookies) -> Response {
    match authenticate(&headers, cookies) {
        Ok(_) => (),
        Err(e) => return e.log_and_response(),
    };

    let mut quotes_map: HashMap<String, Quote> = HashMap::new();
    {
        let conn = get_conn();
        let query = db::queries::ALL_QUOTES_QUERY;
        let mut st = conn.prepare(query).unwrap();
        loop {
            match st.next() {
                Ok(State::Row) => {
                    let quoteid: String = st.read("quote_id").unwrap();
                    let timestamp: i64 = st.read("timestamp").unwrap();
                    let context: Option<String> = st.read("context").ok();

                    let lineid: String = st.read("line_id").unwrap();
                    let linecontent: String = st.read("line_content").unwrap();
                    let linepos = u8::try_from(st.read::<i64, _>("linepos").unwrap()).unwrap();

                    let author_id: String = st.read("author_id").unwrap();
                    let author = Author {
                        id: Ulid::from_string(&author_id).unwrap(),
                        name: st.read("author_name").unwrap(),
                        obfname: st.read("author_obfname").unwrap(),
                    };

                    let line = Line {
                        id: Ulid::from_string(&lineid).unwrap(),
                        content: linecontent,
                        position: linepos,
                        quote: lineid.clone(),
                        author: author_id,
                    };

                    let quote = quotes_map.entry(quoteid.clone()).or_insert_with(|| Quote {
                        id: Ulid::from_string(&quoteid).unwrap(),
                        timestamp,
                        context: context.clone(),
                        lines: Vec::new(),
                        authors: Vec::new(),
                    });

                    quote.lines.push(line);

                    if !quote.authors.iter().any(|a| a.id == author.id) {
                        quote.authors.push(author);
                    }
                }
                Ok(State::Done) => break,
                Err(e) => {
                    error!("Error in db st.next() access -> {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
            }
        }
    };

    return Json(quotes_map.into_values().collect::<Vec<Quote>>()).into_response();
    // return Json(quotes_map).into_response();
}

#[derive(Deserialize)]
struct CreateQuote {
    lines: Vec<CreateQuoteLine>,
    context: Option<String>,
}
#[derive(Deserialize)]
struct CreateQuoteLine {
    content: String,
    author: Ulid,
}
async fn add_quote(
    headers: HeaderMap,
    cookies: Cookies,
    Json(body): Json<CreateQuote>,
) -> Response {
    let actor = match authenticate(&headers, cookies) {
        Ok(actor) => actor,
        Err(e) => return e.log_and_response(),
    };

    match UserPermission::check_permission(&UserPermission::CreateQuotes, &actor.perms) {
        true => (),
        false => return StatusCode::FORBIDDEN.into_response(),
    };

    if body.lines.is_empty() {
        return (StatusCode::BAD_REQUEST, "Some lines are required.").into_response();
    }

    let quoteid = Ulid::new();
    let ts = Utc::now().timestamp();
    {
        let conn = get_conn();
        conn.execute("BEGIN TRANSACTION").unwrap();

        let query = "INSERT INTO quotes VALUES (:id, :ts, :ctx)";
        let mut st = conn.prepare(query).unwrap();
        st.bind((":id", quoteid.to_string().as_str())).unwrap();
        st.bind((":ts", ts)).unwrap();
        match body.context {
            Some(ctx) => st.bind((":ctx", ctx.as_str())).unwrap(),
            None => st.bind((":ctx", None::<&str>)).unwrap(),
        };

        match st.next() {
            Ok(_) => (),
            Err(e) => {
                error!("Could not create quote in db: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }

        let query = "INSERT INTO lines VALUES (:id, :content, :pos, :q, :a)";
        for (i, line) in body.lines.into_iter().enumerate() {
            let id = Ulid::new();
            let mut st = conn.prepare(query).unwrap();
            st.bind((":id", id.to_string().as_str())).unwrap();
            st.bind((":content", line.content.as_str())).unwrap();
            st.bind((":pos", i as i64)).unwrap();
            st.bind((":q", quoteid.to_string().as_str())).unwrap();
            st.bind((":a", line.author.to_string().as_str())).unwrap();

            match st.next() {
                Ok(_) => (),
                Err(e) => {
                    error!("Could not insert line {i} of this quote! {e}");
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
            }
        }

        conn.execute("COMMIT").unwrap();
    }

    return StatusCode::OK.into_response();
}
