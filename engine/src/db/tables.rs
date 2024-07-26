pub static USERS: &str = "
    CREATE TABLE IF NOT EXISTS users (
        id          TEXT NOT NULL UNIQUE PRIMARY KEY,
        name        TEXT NOT NULL UNIQUE,
        pass        TEXT NOT NULL
    )
";

pub static LOGS: &str = "
    CREATE TABLE IF NOT EXISTS logs (
        id          TEXT NOT NULL UNIQUE PRIMARY KEY,
        content     TEXT NOT NULL,
        timestamp   INTEGER
    )
";

pub static AUTHORS: &str = "
    CREATE TABLE IF NOT EXISTS authors (
        id          TEXT NOT NULL UNIQUE PRIMARY KEY,
        name        TEXT NOT NULL
    )
";

pub static LINES: &str = "
    CREATE TABLE IF NOT EXISTS lines (
        id          TEXT NOT NULL UNIQUE PRIMARY KEY,
        content     TEXT NOT NULL,
        position    INTEGER NOT NULL,
        quote       TEXT NOT NULL,
        author      TEXT NOT NULL
    )
";

pub static QUOTES: &str = "
    CREATE TABLE IF NOT EXISTS quotes (
        id          TEXT NOT NULL UNIQUE PRIMARY KEY,
        context     TEXT DEFAULT NULL,
        timestamp   INTEGER
    )
";
