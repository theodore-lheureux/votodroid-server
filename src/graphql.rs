use crate::{
    context::Context,
    graphql::user_resolver::{UserMutation, UserQuery},
};

use juniper::graphql_object;

mod user_resolver;

pub struct QueryRoot;

#[graphql_object(Context = Context,
    description = "Query Root",)]
impl QueryRoot {
    fn api_version() -> &'static str {
        "1.0"
    }
    fn users(&self) -> UserQuery {
        UserQuery
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context,
    description = "Mutation Root",)]
impl MutationRoot {
    fn users(&self) -> UserMutation {
        UserMutation
    }
}
