-- Your SQL goes here
CREATE TABLE gib_pm.users
(
    id       SERIAL PRIMARY KEY,
    username TEXT        NOT NULL UNIQUE,
    password VARCHAR(60) NOT NULL,
    email    TEXT UNIQUE NOT NULL
);

CREATE TABLE gib_pm.packages
(
    id              SERIAL PRIMARY KEY,
    package_name    TEXT UNIQUE NOT NULL,
    publisher       INT         NOT NULL,
    configuration   TEXT        NOT NULL,
    current_version TEXT        NOT NULL
);


CREATE TABLE gib_pm.package_archives
(
    id         SERIAL PRIMARY KEY,
    package_id INT  NOT NULL,
    version    TEXT NOT NULL,
    archive    BYTEA NOT NULL
);
