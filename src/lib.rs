use actix_web::{web, Error, HttpResponse};
use context::GraphQLContext;
use database::get_pool;
use juniper::{graphql_object, EmptySubscription, FieldResult, RootNode};
use juniper_actix::graphql_handler;
use models::user::{RegisterUserInput, User};
use uuid::Uuid;

use crate::services::user::get_user;

pub mod context;
pub mod database;
pub mod models;
pub mod schema;
pub mod services;

pub struct QueryRoot;

#[graphql_object(Context = GraphQLContext,
    description = "Query Root",)]
impl QueryRoot {
    fn api_version() -> &'static str {
        "1.0"
    }
    #[graphql(description = "get a user")]
    fn user(ctx: &GraphQLContext, user_id: Uuid) -> FieldResult<User> {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        Ok(get_user(&mut conn, user_id)?)
    }
}

pub struct MutationRoot;

#[graphql_object(Context = GraphQLContext,
    description = "Mutation Root",)]
impl MutationRoot {
    #[graphql(description = "create a new user")]
    fn register_user(
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

pub type Schema = RootNode<
    'static,
    QueryRoot,
    MutationRoot,
    EmptySubscription<GraphQLContext>,
>;

pub fn schema() -> Schema {
    Schema::new(
        QueryRoot,
        MutationRoot,
        EmptySubscription::<GraphQLContext>::new(),
    )
}

pub async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    data: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = GraphQLContext { pool: get_pool() };
    graphql_handler(&data, &context, req, payload).await
}
