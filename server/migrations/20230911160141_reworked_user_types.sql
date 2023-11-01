-- Add migration script here
DROP TABLE IF EXISTS streamers;
CREATE TABLE
    IF NOT EXISTS orgs (
        id SERIAL PRIMARY KEY NOT NULL,
        org_id VARCHAR(255) NOT NULL UNIQUE,
        name VARCHAR(255) NOT NULL UNIQUE,
        status BOOLEAN NOT NULL DEFAULT FALSE
    );

CREATE TABLE
    IF NOT EXISTS mods (
        id CHAR(36) PRIMARY KEY NOT NULL,
        name VARCHAR(36) NOT NULL,
        password CHAR(97) NOT NULL,
        org_id CHAR(36) NOT NULL,
        api_token CHAR(97) NOT NULL,
        root BOOLEAN NOT NULL DEFAULT FALSE,
        receiver BOOLEAN NOT NULL DEFAULT FALSE
    );