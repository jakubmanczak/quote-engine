use super::get_conn;
use crate::logs::{push_log, LogEntry};
use crate::models::User;
use crate::models::DEFAULT_COLOR;
use crate::permissions::UserPermission;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::Utc;
use tracing::{error, info};
use ulid::Ulid;

const DEFAULT_ADMIN_CREATED: &str = "Default admin account created.";
const REMOVE_DEFAULT_ADMIN: &str = "Please change the password or swap this account!";

pub fn run() {
    let conn = get_conn();
    let user = User {
        id: Ulid::new(),
        name: "admin".to_owned(),
        color: DEFAULT_COLOR.to_owned(),
        picture: String::new(),
        perms: vec![UserPermission::Everything],
    };

    {
        let q = "INSERT INTO users VALUES (:id, :name, :pass, :perms, :color, :picture)";
        let mut statement = conn.prepare(q).unwrap();

        let password = b"admin";
        let salt = SaltString::generate(&mut OsRng);

        let argon = Argon2::default();
        let hash = match argon.hash_password(password, &salt) {
            Ok(hash) => hash.to_string(),
            Err(e) => {
                error!("could not hash default admin password: {e}");
                panic!();
            }
        };

        statement
            .bind((":id", user.id.to_string().as_str()))
            .unwrap();
        statement.bind((":name", user.name.as_str())).unwrap();
        statement.bind((":pass", hash.as_str())).unwrap();
        statement.bind((":color", user.color.as_str())).unwrap();
        statement.bind((":picture", user.picture.as_str())).unwrap();
        statement
            .bind((
                ":perms",
                i64::from(UserPermission::get_bits_from_permissions(&user.perms)),
            ))
            .unwrap();

        match statement.next() {
            Ok(_) => (),
            Err(e) => {
                error!("Could not create default admin account: {e}");
                panic!();
            }
        }
    }

    info!("{}", DEFAULT_ADMIN_CREATED);
    info!("{}", REMOVE_DEFAULT_ADMIN);
    push_log(LogEntry {
        id: Ulid::new(),
        timestamp: Utc::now().timestamp(),
        actor: Ulid::nil(),
        subject: user.id,
        action: crate::logs::LogEvent::UserCreatedBySystem(user),
    });
}
