use juniper::FieldResult;
use uuid::Uuid;

use crate::{
    context::GraphQLContext,
    models::user::{RegisterUserInput, User},
    services::{self, user::get_user},
};

pub struct UserQuery;

#[juniper::graphql_object(Context = GraphQLContext)]
impl UserQuery {
    /// Get a user from their Id (UUID)
    fn get_by_id(ctx: &GraphQLContext, user_id: String) -> FieldResult<User> {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");

        let user_id = Uuid::parse_str(&user_id)?;

        Ok(get_user(&mut conn, user_id)?)
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
