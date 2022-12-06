use actix_web::{web, Error, HttpResponse};
use context::GraphQLContext;
use database::get_pool;
use diesel::RunQueryDsl;
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use juniper_actix::graphql_handler;
use models::user::User;
use schema::users::dsl::*;

pub mod context;
pub mod database;
pub mod models;
pub mod schema;

pub struct Query;
#[graphql_object(context = GraphQLContext)]
impl Query {
    fn api_version() -> &'static str {
        println!("das0");
        "1.0"
    }
    fn get_first_user(ctx: &GraphQLContext) -> User {
        let conn = &mut ctx.pool.get().unwrap();
        let u: User = users.first(conn).expect("msg");
        println!("{}", u.username);
        u
    }
}

pub type Schema = RootNode<
    'static,
    Query,
    EmptyMutation<GraphQLContext>,
    EmptySubscription<GraphQLContext>,
>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<GraphQLContext>::new(),
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
