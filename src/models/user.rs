use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::schema;

use super::types::FieldError;

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
    /// The date and time the user last logged in
    pub last_login: Option<NaiveDateTime>,
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

#[derive(GraphQLObject)]
pub struct UserResponse {
    pub user: Option<User>,
    pub errors: Option<Vec<FieldError>>,
}

impl UserResponse {
    pub fn from_user(user: User) -> UserResponse {
        UserResponse {
            user: Some(user),
            errors: None,
        }
    }
    pub fn from_errors(errors: Vec<FieldError>) -> UserResponse {
        UserResponse {
            user: None,
            errors: Some(errors),
        }
    }
    pub fn from_error(field: String, message: String) -> UserResponse {
        UserResponse {
            user: None,
            errors: Some(vec![FieldError::new(field, message)]),
        }
    }
}
