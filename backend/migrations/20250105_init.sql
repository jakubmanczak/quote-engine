CREATE TABLE users (
    id                  UUID NOT NULL UNIQUE PRIMARY KEY,
    handle              TEXT NOT NULL UNIQUE,
    clearance           SMALLINT NOT NULL,
    attributes          BIGINT NOT NULL,
    
    password_hash       TEXT NOT NULL
);

CREATE TABLE sessions (
    id                  UUID NOT NULL UNIQUE PRIMARY KEY,
    token               TEXT NOT NULL UNIQUE,
    user_id             UUID NOT NULL REFERENCES users (id),
    
    issued              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expiry              TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '1 week',
    last_access         TIMESTAMPTZ DEFAULT NULL,
    revoked             BOOLEAN NOT NULL DEFAULT FALSE,
    revoked_at          TIMESTAMPTZ DEFAULT NULL
);

CREATE TABLE logs (
    id                  UUID NOT NULL UNIQUE PRIMARY KEY,
    timestamp           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    actor_id            UUID NOT NULL REFERENCES users(id),
    subject_id          UUID NOT NULL,
    action              TEXT NOT NULL,
    details             JSONB NOT NULL
);

CREATE TABLE quotes (
    id                  UUID NOT NULL UNIQUE PRIMARY KEY,
    source              TEXT NOT NULL,
    context             TEXT NOT NULL,
    clearance           BIGINT NOT NULL,
    timestamp           TIMESTAMP NOT NULL
);

CREATE TABLE authors (
    id                  UUID NOT NULL UNIQUE PRIMARY KEY,
    fullname            TEXT NOT NULL UNIQUE,
    codename            TEXT NOT NULL UNIQUE,
    bio                 TEXT DEFAULT NULL
);

CREATE TABLE lines (
);