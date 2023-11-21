-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    username VARCHAR(25) NOT NULL,
    email VARCHAR(50) NOT NULL,
    password TEXT NOT NULL,
    PRIMARY KEY (username),
    UNIQUE (username),
    UNIQUE (email)
);

CREATE INDEX idx_username ON users (username);