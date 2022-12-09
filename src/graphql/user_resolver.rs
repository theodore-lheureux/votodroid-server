use regex::Regex;
use uuid::Uuid;

use crate::{
    context::Context,
    models::{
        field_error::FieldError,
        user::{RegisterUserInput, User, UserResponse},
    },
    services::{self, user::get_by_id},
};

pub struct UserQuery;

#[juniper::graphql_object(Context = Context)]
impl UserQuery {
    /// Get a user from their Id (UUID)
    fn get_by_id(ctx: &Context, user_id: String) -> UserResponse {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let user_id = Uuid::parse_str(&user_id);
        let mut errors = vec![];

        if let Err(e) = user_id {
            errors.push(FieldError::new("userId".to_owned(), e.to_string()));
            return UserResponse::from_errors(errors);
        }

        let user = get_by_id(&mut conn, user_id.unwrap());

        match user {
            Ok(user) => UserResponse::from_user(user),
            Err(e) => {
                errors
                    .push(FieldError::new("userId".to_owned(), e.to_string()));
                UserResponse::from_errors(errors)
            }
        }
    }

    fn get_all(ctx: &Context) -> Vec<User> {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        services::user::get_all(&mut conn).unwrap()
    }

    pub fn me(ctx: &Context) -> UserResponse {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let mut errors = vec![];

        let user_id = ctx.session.get::<Uuid>("userId").unwrap();

        if let Some(user_id) = user_id {
            let user = get_by_id(&mut conn, user_id);

            match user {
                Ok(user) => UserResponse::from_user(user),
                Err(e) => {
                    errors.push(FieldError::new(
                        "userId".to_owned(),
                        e.to_string(),
                    ));
                    UserResponse::from_errors(errors)
                }
            }
        } else {
            errors.push(FieldError::new(
                "userId".to_owned(),
                "User is not logged in.".to_owned(),
            ));
            UserResponse::from_errors(errors)
        }
    }
}

pub struct UserMutation;

#[juniper::graphql_object(Context = Context)]
impl UserMutation {
    /// Register a new user
    fn register(
        ctx: &Context,
        mut new_user: RegisterUserInput,
    ) -> UserResponse {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let mut errors = vec![];
        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

        if (new_user.username.len() < 3) || (new_user.username.len() > 20) {
            errors.push(FieldError::new(
                "username".to_owned(),
                "Username must be between 3 and 20 characters.".to_owned(),
            ));
        } else if services::user::get_by_username(&mut conn, &new_user.username)
            .is_ok()
        {
            errors.push(FieldError::new(
                "username".to_owned(),
                "Username already exists.".to_owned(),
            ));
        } else if !new_user.username.chars().all(|c| c.is_alphanumeric()) {
            errors.push(FieldError::new(
                "username".to_owned(),
                "Username must be alphanumeric.".to_owned(),
            ));
        }
        if !email_regex.is_match(&new_user.email) {
            errors.push(FieldError::new(
                "email".to_owned(),
                "Email must be valid.".to_owned(),
            ));
        } else if services::user::get_by_email(&mut conn, &new_user.email)
            .is_ok()
        {
            errors.push(FieldError::new(
                "email".to_owned(),
                "Email already exists.".to_owned(),
            ));
        }
        if new_user.password.len() < 8 {
            errors.push(FieldError::new(
                "password".to_owned(),
                "Password must be at least 8 characters.".to_owned(),
            ));
        }

        if !errors.is_empty() {
            return UserResponse::from_errors(errors);
        }

        UserResponse::from_user(
            services::user::create_user(&mut conn, new_user).unwrap(),
        )
    }

    fn login(
        ctx: &Context,
        username_or_email: String,
        password: String,
    ) -> UserResponse {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let mut errors = vec![];
        let user;

        if username_or_email.contains('@') {
            user = services::user::get_by_email(&mut conn, &username_or_email);
        } else {
            user =
                services::user::get_by_username(&mut conn, &username_or_email);
        }

        match user {
            Ok(user) => {
                if !argon2::verify_encoded(&user.password, password.as_bytes())
                    .unwrap()
                {
                    errors.push(FieldError::new(
                        "password".to_owned(),
                        "Password is incorrect.".to_owned(),
                    ));
                    return UserResponse::from_errors(errors);
                }

                services::user::update_last_login(&mut conn, user.id).unwrap();
                ctx.session.insert("userId", user.id).unwrap();
                UserResponse::from_user(user)
            }
            Err(_) => {
                errors.push(FieldError::new(
                    "usernameOrEmail".to_owned(),
                    "Username or email does not exist.".to_owned(),
                ));
                UserResponse::from_errors(errors)
            }
        }
    }

    fn logout(ctx: &Context) -> bool {
        ctx.session.remove("userId");
        true
    }
}
