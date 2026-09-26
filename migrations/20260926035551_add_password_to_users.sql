-- Add migration script here
ALTER TABLE users ADD COLUMN password TEXT NOT NULL DEFAULT '';
ALTER TABLE users ADD COLUMN username TEXT NOT NULL DEFAULT '';

CREATE UNIQUE INDEX idx_users_username ON users(username);
