use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::schema;

#[derive(Clone, Queryable, GraphQLObject)]
///A user
pub struct User {
    /// The user's id (UUID)
    pub id: Uuid,
    /// The user's username
    pub username: String,
    /// The user's email
    pub email: String,
    #[graphql(skip)]
    pub password: String,
    /// The date and time the user was created
    pub created_at: NaiveDateTime,
    /// The date and time the user was last updated
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLInputObject, Insertable)]
#[diesel(table_name = schema::users)]
pub struct RegisterUserInput {
    /// The user's username
    pub username: String,
    /// The user's email
    pub email: String,
    // The user's password
    pub password: String,
}

pub struct UserResponse {
    pub user: Option<User>,
    pub error: Option<String>,
}
