use chrono::NaiveDateTime;
use diesel::Queryable;
use juniper::GraphQLObject;

#[derive(Clone, Queryable, GraphQLObject)]
///a user
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
