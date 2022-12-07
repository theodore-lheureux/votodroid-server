use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};
use juniper::{GraphQLObject, GraphQLInputObject};

use crate::schema;

#[derive(Clone, Queryable, GraphQLObject)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[graphql(skip)]
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(GraphQLInputObject, Insertable)]
#[diesel(table_name = schema::users)]
pub struct RegisterUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}
