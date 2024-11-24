CREATE TABLE IF NOT EXISTS Admin (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL CHECK (LENGTH(username) <= 255),
    password BLOB NOT NULL,
    email TEXT NOT NULL CHECK (LENGTH(email) <= 255),
    root BOOLEAN NOT NULL,
    last_login_time TIMESTAMP
);
