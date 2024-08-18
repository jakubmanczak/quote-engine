use crate::{
    auth::authenticate,
    db::{
        get_conn,
        users::{get_user_data, GetUserDataInput},
    },
    logs::{push_log, LogEntry, LogEvent},
    models::{User, DEFAULT_COLOR, DEFAULT_PICTURE},
    permissions::{UserPermission, DEFAULT_PERMISSIONS},
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
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

const NO_USERS: &str = "No users found.";

pub fn exported_routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/users/:id", get(get_user_by_id))
        .route("/users/count", get(get_users_count))
        .route("/users", post(post_users))
        .route("/users/:id", patch(patch_user))
        .route("/users/:id/changepassword", patch(patch_user_password))
        .route("/users/:id", delete(delete_user))
}

async fn get_users(headers: HeaderMap, cookies: Cookies) -> Response {
    match authenticate(&headers, cookies) {
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        Ok(_) => (),
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
                perms: UserPermission::get_permissions_from_bits(
                    match u32::try_from(statement.read::<i64, _>("permissions").unwrap()) {
                        Ok(u) => u,
                        Err(e) => {
                            let res = format!("Could not convert db i64 to bitflag u32: {e}");
                            return (StatusCode::INTERNAL_SERVER_ERROR, res).into_response();
                        }
                    },
                ),
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

async fn get_user_by_id(headers: HeaderMap, cookies: Cookies, Path(id): Path<String>) -> Response {
    match authenticate(&headers, cookies) {
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        Ok(_) => (),
    };

    let conn = get_conn();
    let query = "SELECT * FROM users WHERE id = :id";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((":id", id.as_str())).unwrap();

    match statement.next() {
        Ok(State::Row) => {
            return Json(User {
                id: statement.read("id").unwrap(),
                name: statement.read("name").unwrap(),
                color: statement.read("color").unwrap(),
                picture: statement.read("picture").unwrap(),
                perms: UserPermission::get_permissions_from_bits(
                    match u32::try_from(statement.read::<i64, _>("permissions").unwrap()) {
                        Ok(u) => u,
                        Err(e) => {
                            let res = format!("Could not convert db i64 to bitflag u32: {e}");
                            return (StatusCode::INTERNAL_SERVER_ERROR, res).into_response();
                        }
                    },
                ),
            })
            .into_response()
        }
        Ok(State::Done) => return (StatusCode::BAD_REQUEST, "No such user found.").into_response(),
        Err(e) => {
            error!("Error in GET /users/:id: {e}");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }
}

async fn get_users_count(headers: HeaderMap, cookies: Cookies) -> Response {
    match authenticate(&headers, cookies) {
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
        Ok(_) => (),
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
async fn post_users(
    headers: HeaderMap,
    cookies: Cookies,
    Json(body): Json<CreateUser>,
) -> Response {
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };

    match UserPermission::check_permission(&UserPermission::CreateUsers, &actor.perms) {
        true => (),
        false => return StatusCode::FORBIDDEN.into_response(),
    }

    let subject: User;
    {
        let ulid = Ulid::new().to_string();
        let conn = get_conn();
        let query = "INSERT INTO users VALUES (:id, :name, :pass, :perms, :color, :pic)";
        let mut st = conn.prepare(query).unwrap();

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
            perms: Vec::from(DEFAULT_PERMISSIONS),
        };

        st.bind((":id", subject.id.as_str())).unwrap();
        st.bind((":name", subject.name.as_str())).unwrap();
        st.bind((":pass", hash.as_str())).unwrap();
        st.bind((
            ":perms",
            i64::from(UserPermission::get_bits_from_permissions(&subject.perms)),
        ))
        .unwrap();
        st.bind((":color", subject.color.as_str())).unwrap();
        st.bind((":pic", subject.picture.as_str())).unwrap();

        match st.next() {
            Ok(_) => (),
            Err(e) => {
                error!("Could not create account: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    push_log(LogEntry {
        id: Ulid::new().to_string(),
        timestamp: Utc::now().timestamp(),
        actor: actor.id,
        subject: subject.id.clone(),
        action: LogEvent::UserCreated(subject.clone()),
    });
    return Json(subject).into_response();
}

#[derive(Deserialize, Serialize)]
struct PatchUser {
    name: Option<String>,
    color: Option<String>,
    picture: Option<String>,
    perms: Option<Vec<UserPermission>>,
}
async fn patch_user(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<String>,
    Json(body): Json<PatchUser>,
) -> Response {
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };

    let subject = match get_user_data(GetUserDataInput::Id(id.clone())) {
        Ok(user) => user,
        Err(e) => match e {
            crate::error::Error::NoRowsError(str) => {
                return (StatusCode::BAD_REQUEST, str).into_response();
            }
            _ => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        },
    };

    let mut checkperms: Vec<UserPermission> = Vec::new();
    {
        use UserPermission::*;
        let oldperm_everything = subject.perms.contains(&Everything);
        let newperm_everything = body.perms.clone().is_some_and(|v| v.contains(&Everything));
        match actor.id == id {
            true => checkperms.push(MutateOwnUser),
            false => checkperms.push(MutateUsers),
        }
        if body.perms.is_some() {
            checkperms.push(MutateUsersPermissions);
        }
        if oldperm_everything || newperm_everything {
            checkperms.push(Everything);
        }
    }
    for perm in checkperms {
        match UserPermission::check_permission(&perm, &actor.perms) {
            true => (),
            false => {
                return (
                    StatusCode::FORBIDDEN,
                    format!("Lacking permission: {:?}", perm),
                )
                    .into_response()
            }
        }
    }

    let q =
        "UPDATE users SET name = :n, color = :c, picture = :pic, permissions = :prm WHERE id = :id";
    let name = match body.name.clone() {
        Some(n) => n,
        None => subject.name.clone(),
    };
    let color = match body.color.clone() {
        Some(c) => c,
        None => subject.color.clone(),
    };
    let picture = match body.picture.clone() {
        Some(pic) => pic,
        None => subject.picture.clone(),
    };
    let perms = match body.perms.clone() {
        Some(perms) => UserPermission::get_bits_from_permissions(&perms),
        None => UserPermission::get_bits_from_permissions(&subject.perms),
    };
    {
        let conn = get_conn();
        let mut st = conn.prepare(q).unwrap();
        st.bind((":id", id.as_str())).unwrap();
        st.bind((":n", name.as_str())).unwrap();
        st.bind((":c", color.as_str())).unwrap();
        st.bind((":pic", picture.as_str())).unwrap();
        st.bind((":prm", i64::from(perms))).unwrap();

        match st.next() {
            Ok(_) => (),
            Err(e) => {
                error!("Could not PATCH user: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
            }
        }
    }

    if let Some(name) = body.name {
        push_log(LogEntry {
            id: Ulid::new().to_string(),
            timestamp: Utc::now().timestamp(),
            actor: actor.id.clone(),
            subject: subject.id.clone(),
            action: LogEvent::UserNameUpdated {
                old_name: subject.name.clone(),
                new_name: name,
            },
        });
    }
    if let Some(color) = body.color {
        push_log(LogEntry {
            id: Ulid::new().to_string(),
            timestamp: Utc::now().timestamp(),
            actor: actor.id.clone(),
            subject: subject.id.clone(),
            action: LogEvent::UserColorUpdated {
                old_color: subject.color.clone(),
                new_color: color,
            },
        });
    }
    if let Some(picture) = body.picture {
        push_log(LogEntry {
            id: Ulid::new().to_string(),
            timestamp: Utc::now().timestamp(),
            actor: actor.id.clone(),
            subject: subject.id.clone(),
            action: LogEvent::UserPictureUpdated {
                old_picture: subject.picture.clone(),
                new_picture: picture,
            },
        });
    }
    if let Some(perms) = body.perms {
        push_log(LogEntry {
            id: Ulid::new().to_string(),
            timestamp: Utc::now().timestamp(),
            actor: actor.id.clone(),
            subject: subject.id.clone(),
            action: LogEvent::UserPermissionsUpdated {
                old_perms: subject.perms.clone(),
                new_perms: perms,
            },
        });
    }
    return StatusCode::OK.into_response();
}

#[derive(Deserialize)]
struct PatchUserPassword {
    pass: String,
}
async fn patch_user_password(
    headers: HeaderMap,
    cookies: Cookies,
    Path(id): Path<String>,
    Json(body): Json<PatchUserPassword>,
) -> Response {
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };

    {
        let perm = match actor.id == id {
            true => UserPermission::MutateOwnUser,
            false => UserPermission::MutateUsersPasswords,
        };
        match UserPermission::check_permission(&perm, &actor.perms) {
            true => (),
            false => return StatusCode::FORBIDDEN.into_response(),
        }
    }

    let hash: String;
    {
        let argon = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        hash = match argon.hash_password(body.pass.as_bytes(), &salt) {
            Ok(h) => h.to_string(),
            Err(e) => {
                error!("Could not hash new user password: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }
    }

    {
        let conn = get_conn();
        let q = "UPDATE users SET pass = :p WHERE id = :id";
        let mut statement = conn.prepare(q).unwrap();
        statement.bind((":id", id.as_str())).unwrap();
        statement.bind((":p", hash.as_str())).unwrap();

        match statement.next() {
            Ok(_) => (),
            Err(e) => {
                error!("Could not update password: {e}");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }
    }

    push_log(LogEntry {
        id: Ulid::new().to_string(),
        timestamp: Utc::now().timestamp(),
        actor: actor.id.clone(),
        subject: id.clone(),
        action: LogEvent::UserPasswordUpdated,
    });
    (StatusCode::OK, "Password changed.").into_response()
}

async fn delete_user(headers: HeaderMap, cookies: Cookies, Path(id): Path<String>) -> Response {
    let actor = match authenticate(&headers, cookies) {
        Ok(user) => user,
        Err(e) => return (StatusCode::UNAUTHORIZED, e.to_string()).into_response(),
    };

    match UserPermission::check_permission(&UserPermission::DeleteUsers, &actor.perms) {
        true => (),
        false => return StatusCode::FORBIDDEN.into_response(),
    }

    let subject = match get_user_data(GetUserDataInput::Id(id.clone())) {
        Ok(user) => user,
        Err(e) => {
            error!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    };

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

    push_log(LogEntry {
        id: Ulid::new().to_string(),
        timestamp: Utc::now().timestamp(),
        actor: actor.id,
        subject: subject.id.clone(),
        action: LogEvent::UserDeleted(subject),
    });
    return StatusCode::NO_CONTENT.into_response();
}
