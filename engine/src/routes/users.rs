use crate::{
    auth::{get_auth_from_header, validate::validate_basic_auth, AuthType},
    db::{
        get_conn,
        log_events::{
            LogEvents::{UserCreated, UserDeleted},
            LogUserInfo,
        },
        push_log,
    },
    models::{User, DEFAULT_COLOR, DEFAULT_PICTURE},
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use sqlite::State;
use tracing::error;
use ulid::Ulid;

const NO_USERS: &str = "No users found.";

pub fn exported_routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/count", get(get_users_count))
        .route("/users", post(post_users))
        .route("/users/:id", delete(delete_user))
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
    let query = "SELECT * FROM users";
    let mut statement = conn.prepare(query).unwrap();

    let mut users: Vec<User> = Vec::new();
    loop {
        match statement.next() {
            Ok(State::Row) => users.push(User {
                id: statement.read("id").unwrap(),
                name: statement.read("name").unwrap(),
                color: statement.read("color").unwrap(),
                picture: statement.read("picture").unwrap(),
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

async fn get_users_count(headers: HeaderMap) -> Response {
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
    let query = "SELECT COUNT(*) FROM users";
    let mut statement = conn.prepare(query).unwrap();
    match statement.next() {
        Ok(State::Row) => {
            let count: i64 = statement.read(0).unwrap();
            return count.to_string().into_response();
        }
        Ok(State::Done) => return (StatusCode::NOT_FOUND, NO_USERS).into_response(),
        Err(e) => {
            error!("Error in GET /users/count: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    }
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    pass: String,
    color: Option<String>,
    picture: Option<String>,
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

    let actor: User;
    {
        let conn = get_conn();
        let actorquery = "SELECT * FROM users WHERE name = :name";
        let mut actorstatement = conn.prepare(actorquery).unwrap();
        actorstatement.bind((":name", auth.user.as_str())).unwrap();
        match actorstatement.next() {
            Ok(State::Row) => {
                actor = User {
                    id: actorstatement.read("id").unwrap(),
                    name: auth.user,
                    color: actorstatement.read("color").unwrap(),
                    picture: actorstatement.read("picture").unwrap(),
                }
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

    let subject: User;
    {
        let ulid = Ulid::new().to_string();
        let conn = get_conn();
        let query = "INSERT INTO users VALUES (:id, :name, :pass, :color, :picture)";
        let mut statement = conn.prepare(query).unwrap();

        let argon = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let hash = match argon.hash_password(body.pass.as_bytes(), &salt) {
            Ok(hash) => hash.to_string(),
            Err(e) => {
                error!("Could not hash new user password: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        };

        subject = User {
            id: ulid,
            name: body.name,
            color: match body.color {
                Some(c) => c,
                None => DEFAULT_COLOR.to_owned(),
            },
            picture: match body.picture {
                Some(p) => p,
                None => DEFAULT_PICTURE.to_owned(),
            },
        };

        statement.bind((":id", subject.id.as_str())).unwrap();
        statement.bind((":name", subject.name.as_str())).unwrap();
        statement.bind((":pass", hash.as_str())).unwrap();
        statement.bind((":color", subject.color.as_str())).unwrap();
        statement
            .bind((":picture", subject.picture.as_str()))
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
        actor,
        subject: subject.clone(),
    }));
    return Json(subject).into_response();
}

async fn delete_user(headers: HeaderMap, Path(id): Path<String>) -> Response {
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

    let actor: User;
    {
        let conn = get_conn();
        let actorquery = "SELECT * FROM users WHERE name = :name";
        let mut actorstatement = conn.prepare(actorquery).unwrap();
        actorstatement.bind((":name", auth.user.as_str())).unwrap();
        match actorstatement.next() {
            Ok(State::Row) => {
                actor = User {
                    id: actorstatement.read("id").unwrap(),
                    name: actorstatement.read("name").unwrap(),
                    color: actorstatement.read("color").unwrap(),
                    picture: actorstatement.read("picture").unwrap(),
                };
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

    let subject: User;
    {
        let conn = get_conn();
        let subjectquery = "SELECT * FROM users WHERE id = :id";
        let mut subjectstatement = conn.prepare(subjectquery).unwrap();
        subjectstatement.bind((":id", id.as_str())).unwrap();
        match subjectstatement.next() {
            Ok(State::Row) => {
                subject = User {
                    id: subjectstatement.read("id").unwrap(),
                    name: subjectstatement.read("name").unwrap(),
                    color: subjectstatement.read("color").unwrap(),
                    picture: subjectstatement.read("picture").unwrap(),
                };
            }
            Ok(State::Done) => {
                let res = format!("No user with id {id} found.");
                return (StatusCode::BAD_REQUEST, res).into_response();
            }
            Err(e) => {
                error!("Could not search for desired user's name: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    {
        let conn = get_conn();
        let query = "DELETE FROM users WHERE id = :id";
        let mut statement = conn.prepare(query).unwrap();
        statement.bind((":id", id.as_str())).unwrap();

        match statement.next() {
            Ok(_) => (),
            Err(e) => {
                error!("Could not delete user: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    push_log(UserDeleted(LogUserInfo { actor, subject }));
    return StatusCode::NO_CONTENT.into_response();
}
