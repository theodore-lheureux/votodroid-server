use super::database::PostgresPool;

#[derive(Clone)]
pub struct GraphQLContext {
    pub pool: PostgresPool,
}
impl juniper::Context for GraphQLContext {}
