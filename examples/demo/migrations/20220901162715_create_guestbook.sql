-- Add migration script here
CREATE TABLE IF NOT EXISTS guestbook (
    id      SERIAL PRIMARY KEY,
    author  VARCHAR(32) NOT NULL,
    message VARCHAR(280) NOT NULL
);
