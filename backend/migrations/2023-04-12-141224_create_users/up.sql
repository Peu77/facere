-- Your SQL goes here
CREATE TABLE users
(
    uuid       UUID PRIMARY KEY,
    name       VARCHAR   NOT NULL,
    email      VARCHAR   NOT NULL,
    password   VARCHAR   NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)