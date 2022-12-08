use diesel::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use uuid::Uuid;

use crate::models::user::InsertableUser;
use crate::schema::users::dsl::*;
use crate::{
    models::user::{RegisterUserInput, User},
    schema::users,
};

pub fn create_user(
    conn: &mut PgConnection,
    new_user: RegisterUserInput,
) -> QueryResult<User> {
    let user_salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let new_user = InsertableUser {
        username: new_user.username,
        email: new_user.email,
        salt: &user_salt,
        password: argon2::hash_encoded(
            new_user.password.as_bytes(),
            user_salt.as_bytes(),
            &argon2::Config::default(),
        )
        .unwrap(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn get_by_id(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<User> {
    users.find(user_id).first(conn)
}

pub fn get_all(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
    users.load(conn)
}

pub fn get_by_username(
    conn: &mut PgConnection,
    name: &String,
) -> QueryResult<User> {
    users.filter(username.eq(name)).first(conn)
}

pub fn get_by_email(
    conn: &mut PgConnection,
    user_email: &String,
) -> QueryResult<User> {
    users.filter(email.eq(user_email)).first(conn)
}
