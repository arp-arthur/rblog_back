-- Your SQL goes here
CREATE TABLE posts (
    post_id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    is_published BOOLEAN NOT NULL DEFAULT FALSE
)