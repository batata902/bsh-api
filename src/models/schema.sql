CREATE TABLE IF NOT EXISTS users(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nickname TEXT DEFAULT 'anonymous',
    nickcolor TEXT DEFAULT '#ffffff',
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
);
