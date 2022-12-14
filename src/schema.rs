// @generated automatically by Diesel CLI.

diesel::table! {
    questions (id) {
        id -> Uuid,
        text -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_login -> Nullable<Timestamp>,
    }
}

diesel::table! {
    votes (id) {
        id -> Uuid,
        value -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Uuid,
        question_id -> Uuid,
    }
}

diesel::joinable!(questions -> users (user_id));
diesel::joinable!(votes -> questions (question_id));
diesel::joinable!(votes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    questions,
    users,
    votes,
);
