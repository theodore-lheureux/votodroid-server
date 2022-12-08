use diesel::{PgConnection, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

use crate::schema::users::dsl::*;
use crate::{
    models::user::{RegisterUserInput, User},
    schema::users,
};

pub fn create_user(
    conn: &mut PgConnection,
    new_user: RegisterUserInput,
) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_user(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<User> {
    users.find(user_id).first(conn)
}
