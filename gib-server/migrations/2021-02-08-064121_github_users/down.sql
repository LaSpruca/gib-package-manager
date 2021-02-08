-- This file should undo anything in `up.sql`
DROP TABLE gib_pm.users;

CREATE TABLE gib_pm.users
(
    id       SERIAL PRIMARY KEY,
    username TEXT        NOT NULL UNIQUE,
    password VARCHAR(60) NOT NULL,
    email    TEXT UNIQUE NOT NULL
);
