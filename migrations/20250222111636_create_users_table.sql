-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id UUID NOT NULL,
    PRIMARY KEY (id),
    username TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    created_at timestamptz NOT NULL
);