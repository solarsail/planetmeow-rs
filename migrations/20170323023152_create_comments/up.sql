-- Your SQL goes here
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    vid INT REFERENCES visitors(id),
    pid INT REFERENCES posts(id),
    body TEXT NOT NULL,
    created TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    last_edited TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT (NOW() AT TIME ZONE 'UTC'),
    deleted BOOLEAN NOT NULL DEFAULT 'f'
)
