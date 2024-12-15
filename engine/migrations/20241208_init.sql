CREATE TABLE IF NOT EXISTS users (
    id TEXT NOT NULL UNIQUE PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    pass TEXT NOT NULL,
    clearance INTEGER NOT NULL,
    attributes INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id TEXT NOT NULL UNIQUE PRIMARY KEY,
    token TEXT NOT NULL UNIQUE,
    user TEXT NOT NULL,
    issued INTEGER NOT NULL,
    expiry INTEGER NOT NULL,
    lastaccess INTEGER NOT NULL
    -- ^^ potentially expand with valid access ip list,
    -- login geolocation etc.
);

CREATE TABLE IF NOT EXISTS logs (
    id TEXT NOT NULL UNIQUE PRIMARY KEY,
    timestamp INTEGER,
    actor TEXT,
    subject TEXT,
    action TEXT,
    details TEXT
);

CREATE TABLE IF NOT EXISTS quotes (
    id TEXT NOT NULL UNIQUE PRIMARY KEY,
    clearance INTEGER NOT NULL,
    context TEXT DEFAULT NULL,
    timestamp INTEGER
);

CREATE TABLE IF NOT EXISTS lines (
    id TEXT NOT NULL UNIQUE PRIMARY KEY,
    content TEXT NOT NULL,
    -- ^^ what was actually said
    position INTEGER NOT NULL,
    -- ^^ quote line order (1,2,3...)
    quote TEXT NOT NULL,
    -- ^^ id of quote this line belongs to
    author TEXT NOT NULL
    -- ^^ id of author who said this line
);

CREATE TABLE IF NOT EXISTS authors (
    id TEXT NOT NULL UNIQUE PRIMARY KEY,
    name TEXT NOT NULL,
    obfname TEXT NOT NULL
    -- ^^ hidden name, or codename
);

-- „Willard Van Orman Quine... to był filozof z mojej młodości, tj. z czasów kiedyśmy ze sfinksem chodzili do przedszkola...”
-- ~ prof. wisniewski, 2024.11.15 12:37
