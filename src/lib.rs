use actix_web::{web, Error, HttpResponse};
use context::GraphQLContext;
use database::get_pool;
use graphql::{MutationRoot, QueryRoot};
use juniper::{EmptySubscription, RootNode};
use juniper_actix::graphql_handler;

mod context;
mod database;
mod graphql;
mod models;
mod schema;
mod services;

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
