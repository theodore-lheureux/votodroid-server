use crate::{
    context::GraphQLContext,
    graphql::user_resolver::{UserMutation, UserQuery},
};

use juniper::graphql_object;

mod user_resolver;

pub struct QueryRoot;

#[graphql_object(Context = GraphQLContext,
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

#[graphql_object(Context = GraphQLContext,
    description = "Mutation Root",)]
impl MutationRoot {
    fn users(&self) -> UserMutation {
        UserMutation
    }
}
