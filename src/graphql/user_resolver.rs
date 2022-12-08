use juniper::FieldResult;
use uuid::Uuid;

use crate::{
    context::GraphQLContext,
    models::{
        field_error::FieldError,
        user::{RegisterUserInput, User, UserResponse},
    },
    services::{self, user::get_by_id},
};

pub struct UserQuery;

#[juniper::graphql_object(Context = GraphQLContext)]
impl UserQuery {
    /// Get a user from their Id (UUID)
    fn get_by_id(ctx: &GraphQLContext, user_id: String) -> UserResponse {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let user_id = Uuid::parse_str(&user_id);
        let mut errors = vec![];

        if let Err(e) = user_id {
            errors.push(FieldError::new("userId".to_owned(), e.to_string()));
            return UserResponse::build(None, Some(errors));
        }

        let user = get_by_id(&mut conn, user_id.unwrap());

        match user {
            Ok(user) => UserResponse::build(Some(user), None),
            Err(e) => {
                errors
                    .push(FieldError::new("userId".to_owned(), e.to_string()));
                UserResponse::build(None, Some(errors))
            }
        }
    }
}

pub struct UserMutation;

#[juniper::graphql_object(Context = GraphQLContext)]
impl UserMutation {
    #[graphql(description = "create a new user")]
    fn register(
        ctx: &GraphQLContext,
        new_user: RegisterUserInput,
    ) -> FieldResult<User> {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        Ok(services::user::create_user(&mut conn, new_user)?)
    }
}
