CREATE TABLE IF NOT EXISTS Admin (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE CHECK (LENGTH(username) <= 256),
    password BLOB NOT NULL,
    email TEXT NOT NULL CHECK (LENGTH(email) <= 256),
    root BOOLEAN NOT NULL,
    last_login_time TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Customer (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT NOT NULL CHECK (LENGTH(email) <= 256),
    name TEXT NOT NULL CHECK (LENGTH(name) <= 256),
    phone TEXT NOT NULL CHECK (LENGTH(phone) <= 32),
    last_reservation_id INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS Reservation (
    id INTEGER PRIMARY KEY,
    restaurant_id INTEGER NOT NULL,
    customer_id INTEGER NOT NULL,
    table_size INTEGER NOT NULL,
    reservation_time TIMESTAMP NOT NULL,
    notes TEXT CHECK (LENGTH(notes) <= 512),
    status TEXT NOT NULL CHECK (LENGTH(status) <= 32)
);

CREATE TABLE IF NOT EXISTS Restaurant (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL CHECK (LENGTH(name) <= 256),
    max_capacity INTEGER NOT NULL,
    location TEXT NOT NULL CHECK (LENGTH(location) <= 512),
    active BOOLEAN NOT NULL
);
