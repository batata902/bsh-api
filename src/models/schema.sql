CREATE TABLE IF NOT EXISTS users(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    refresh TEXT,
    nickname TEXT DEFAULT 'anonymous',
    nickcolor TEXT DEFAULT '#ffffff',
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);
