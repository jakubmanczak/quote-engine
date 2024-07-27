use crate::{
    auth::{get_auth_from_header, validate::validate_basic_auth, AuthType},
    db::{
        get_conn,
        log_events::{LogEvents::UserCreated, LogUserInfo},
        push_log,
    },
    models::User,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlite::State;
use tracing::error;
use ulid::Ulid;

const NO_USERS: &str = "NO USERS FOUND.";

pub fn exported_routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users", post(post_users))
}

async fn get_users(headers: HeaderMap) -> Response {
    match get_auth_from_header(&headers) {
        Some(auth) => match auth {
            AuthType::Basic(auth) => match validate_basic_auth(&auth) {
                true => (),
                false => return StatusCode::UNAUTHORIZED.into_response(),
            },
            _ => return StatusCode::UNAUTHORIZED.into_response(),
        },
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let conn = get_conn();
    let query = "SELECT id, name FROM users";
    let mut statement = conn.prepare(query).unwrap();

    let mut users: Vec<User> = Vec::new();
    loop {
        match statement.next() {
            Ok(State::Row) => users.push(User {
                id: statement.read::<String, _>("id").unwrap(),
                name: statement.read::<String, _>("name").unwrap(),
            }),
            Ok(State::Done) => match users.is_empty() {
                true => return (StatusCode::NOT_FOUND, NO_USERS).into_response(),
                false => return Json(users).into_response(),
            },
            Err(e) => {
                error!("Error in GET /users: {e}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    }
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    pass: String,
}
async fn post_users(headers: HeaderMap, Json(body): Json<CreateUser>) -> Response {
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

    let actorid: String;
    let ulid = Ulid::new().to_string();

    {
        let conn = get_conn();
        let actorquery = "SELECT id FROM users WHERE name = :name";
        let mut actorstatement = conn.prepare(actorquery).unwrap();
        actorstatement.bind((":name", auth.user.as_str())).unwrap();
        match actorstatement.next() {
            Ok(State::Row) => {
                actorid = actorstatement.read("id").unwrap();
            }
            Ok(State::Done) => {
                error!("Actor was authenticated but not present in users?");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
            Err(e) => {
                error!("{e}");
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    }

    {
        let conn = get_conn();
        let query = "INSERT INTO users VALUES (:id, :name, :pass)";
        let mut statement = conn.prepare(query).unwrap();

        let argon = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hash = match argon.hash_password(body.pass.as_bytes(), &salt) {
            Ok(hash) => hash,
            Err(e) => {
                error!("Could not hash new user password: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        };

        statement.bind((":id", ulid.as_str())).unwrap();
        statement.bind((":name", body.name.as_str())).unwrap();
        statement
            .bind((":pass", hash.to_string().as_str()))
            .unwrap();

        match statement.next() {
            Ok(_) => (),
            Err(e) => {
                error!("Could not create account: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    push_log(UserCreated(LogUserInfo {
        actor: User {
            id: actorid,
            name: auth.user,
        },
        subject: User {
            id: ulid.clone(),
            name: body.name.clone(),
        },
    }));
    return Json(User {
        id: ulid,
        name: body.name,
    })
    .into_response();
}
