pub static USERS: &str = "
    CREATE TABLE IF NOT EXISTS users (
        id          TEXT NOT NULL UNIQUE PRIMARY KEY,
        name        TEXT NOT NULL UNIQUE,
        pass        TEXT NOT NULL,
        -- permissions bitflags
        permissions INTEGER NOT NULL,
        -- accent color
        color       TEXT NOT NULL,
        -- profile picture
        picture     TEXT NOT NULL
    )
";

pub static LOGS: &str = "
    CREATE TABLE IF NOT EXISTS logs (
        id          TEXT NOT NULL UNIQUE PRIMARY KEY,
        timestamp   INTEGER,
        actor       TEXT,
        subject     TEXT,
        action      TEXT,
        details     TEXT
    )
";

pub static AUTHORS: &str = "
    CREATE TABLE IF NOT EXISTS authors (
        id          TEXT NOT NULL UNIQUE PRIMARY KEY,
        name        TEXT NOT NULL,
        -- obfuscated name
        obfname     TEXT NOT NULL
    )
";

pub static LINES: &str = "
    CREATE TABLE IF NOT EXISTS lines (
        id          TEXT NOT NULL UNIQUE PRIMARY KEY,
        -- what was actually said
        content     TEXT NOT NULL,
        -- quote line order (1,2,3...)
        position    INTEGER NOT NULL,
        -- id of quote this line belongs to
        quote       TEXT NOT NULL,
        -- id of author who said this line
        author      TEXT NOT NULL
    )
";

pub static QUOTES: &str = "
    CREATE TABLE IF NOT EXISTS quotes (
        id          TEXT NOT NULL UNIQUE PRIMARY KEY,
        timestamp   INTEGER,
        context     TEXT DEFAULT NULL
    )
";
