use actix_session::Session;

use crate::shared::Shared;

use super::database::PostgresPool;

#[derive(Clone)]
pub struct Context {
    pub pool: PostgresPool,
    pub session: Shared<Session>,
}
impl juniper::Context for Context {}
