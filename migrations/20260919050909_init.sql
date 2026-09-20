CREATE TABLE users (
    id         BLOB PRIMARY KEY NOT NULL,
    name       TEXT NOT NULL,
    created_at DATETIME NOT NULL
);

CREATE TABLE rooms (
    id          BLOB PRIMARY KEY NOT NULL,
    name        TEXT NOT NULL,
    max_members INTEGER NOT NULL DEFAULT 2,
    created_at  DATETIME NOT NULL,
    updated_at  DATETIME NOT NULL
);

CREATE TABLE room_members (
    room_id   BLOB NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    user_id   BLOB NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    joined_at DATETIME NOT NULL,
    PRIMARY KEY (room_id, user_id)
);

CREATE TABLE messages (
    id         BLOB PRIMARY KEY NOT NULL,
    room_id    BLOB NOT NULL REFERENCES rooms(id),
    sender_id  BLOB NOT NULL REFERENCES users(id),
    content    TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);