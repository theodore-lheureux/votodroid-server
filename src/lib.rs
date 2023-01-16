use actix_web::{web, Error, HttpResponse};
use context::Context;
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
mod shared;

pub type Schema =
    RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::<Context>::new())
}

pub async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    data: web::Data<Schema>,
    session: actix_session::Session,
) -> Result<HttpResponse, Error> {
    let context = Context {
        pool: get_pool(),
        session: shared::Shared::new(session),
    };
    graphql_handler(&data, &context, req, payload).await
}
