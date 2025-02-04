CREATE TABLE IF NOT EXISTS Admin (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL UNIQUE CHECK (LENGTH(username) <= 256),
    password BLOB NOT NULL,
    email TEXT NOT NULL CHECK (LENGTH(email) <= 256),
    root BOOLEAN NOT NULL,
    last_login_time TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Reservation (
    id INTEGER PRIMARY KEY,
    book_ref TEXT NOT NULL UNIQUE CHECK (LENGTH(book_ref) <= 16),
    restaurant_id INTEGER NOT NULL,
    customer_email TEXT NOT NULL CHECK (LENGTH(customer_email) <= 256),
    customer_name TEXT NOT NULL CHECK (LENGTH(customer_name) <= 256),
    customer_phone TEXT NOT NULL CHECK (LENGTH(customer_phone) <= 32),
    table_size INTEGER NOT NULL CHECK (table_size > 0), -- Ensure valid table size
    reservation_time TIMESTAMP NOT NULL,
    notes TEXT CHECK (LENGTH(notes) <= 512),
    status TEXT NOT NULL CHECK (status IN ('Pending', 'Confirmed', 'Cancelled')), -- Ensure valid status values
    assigned_table TEXT CHECK (LENGTH(assigned_table) <= 8),
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS Restaurant (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL CHECK (LENGTH(name) <= 256),
    max_capacity INTEGER NOT NULL,
    location TEXT NOT NULL CHECK (LENGTH(location) <= 512),
    active BOOLEAN NOT NULL
);
