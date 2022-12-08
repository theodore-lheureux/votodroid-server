-- Your SQL goes here
CREATE TABLE users (
    id uuid DEFAULT uuid_generate_v4(),
    username VARCHAR(20) NOT NULL UNIQUE,
    email VARCHAR(254) NOT NULL UNIQUE,
    password VARCHAR(512) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id)
)