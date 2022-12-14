-- Your SQL goes here
CREATE TABLE questions (
    id uuid DEFAULT uuid_generate_v4(),
    text VARCHAR(128) NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    user_id uuid NOT NULL references users(id),
    PRIMARY KEY (id)
)