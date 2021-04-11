-- Your SQL goes here
DROP TABLE gib_pm.users;

CREATE TABLE gib_pm.users
(
    id         SERIAL PRIMARY KEY,
    username   TEXT        NOT NULL UNIQUE,
    email      TEXT UNIQUE NOT NULL,
    avatar_url TEXT        NOT NULL
);

CREATE TABLE gib_pm.user_tokens
(
    id      SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    expiry  DATE NOT NULL
);
