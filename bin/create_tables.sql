CREATE TABLE IF NOT EXISTS users (
    id INTEGER UNIQUE PRIMARY KEY,
    user_name TEXT NOT NULL,
    mail_addr TEXT NOT NULL,
    pass_hash TEXT NOT NULL
);
