CREATE TABLE users (
    id   BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE rooms (
    id   BLOB PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE messages (
    id         BLOB PRIMARY KEY NOT NULL,
    room_id    BLOB NOT NULL REFERENCES rooms(id),
    sender_id  BLOB NOT NULL REFERENCES users(id),
    content    TEXT NOT NULL,
    created_at DATETIME NOT NULL
);

CREATE INDEX idx_messages_room_created ON messages(room_id, created_at);