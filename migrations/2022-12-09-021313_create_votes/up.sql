-- Your SQL goes here
CREATE TABLE votes (
    id uuid DEFAULT uuid_generate_v4(),
    value INTEGER NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    user_id uuid NOT NULL references users(id),
    question_id uuid NOT NULL references questions(id),
    PRIMARY KEY (id)
)