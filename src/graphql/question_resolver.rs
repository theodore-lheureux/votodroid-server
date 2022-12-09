use uuid::Uuid;

use crate::{
    context::Context,
    models::{
        field_error::FieldError,
        question::{QuestionInput, QuestionResponse},
    },
    services::{self, question::get_by_id},
};

pub struct QuestionQuery;

#[juniper::graphql_object(Context = Context)]
impl QuestionQuery {
    fn get_by_id(ctx: &Context, question_id: String) -> QuestionResponse {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let question_id = Uuid::parse_str(&question_id);
        let mut errors = vec![];

        if let Err(e) = question_id {
            errors
                .push(FieldError::new("questionId".to_owned(), e.to_string()));
            return QuestionResponse::from_errors(errors);
        }

        let question = get_by_id(&mut conn, question_id.unwrap());

        match question {
            Ok(question) => QuestionResponse::from_question(question),
            Err(e) => {
                errors.push(FieldError::new(
                    "questionId".to_owned(),
                    e.to_string(),
                ));
                QuestionResponse::from_errors(errors)
            }
        }
    }
    fn get_paginated(
        ctx: &Context,
        cursor: Option<String>,
        limit: Option<i32>,
    ) -> Vec<Question> {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let cursor = Uuid::parse_str(&c);
        let limit = limit.unwrap_or(20);
        let mut errors = vec![];

        if let Some(Err(e)) = cursor {
            errors
                .push(FieldError::new("cursor".to_owned(), e.to_string()));
            return QuestionResponse::from_errors(errors);
        }

        let questions = services::question::get_paginated(
            &mut conn,
            cursor.map(|c| c.unwrap()),
            limit,
        );

        match questions {
            Ok(questions) => questions,
            Err(e) => {
                errors.push(FieldError::new(
                    "questions".to_owned(),
                    e.to_string(),
                ));
                QuestionResponse::from_errors(errors)
            }
        }
    }
    fn delete_all_by_user(ctx: &Context) -> bool {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let mut errors = vec![];

        let user_id = ctx
            .session
            .get::<Uuid>("userId")
            .expect("Failed to get user id from session.");

        if let Some(user_id) = user_id {
            let user = services::user::get_by_id(&mut conn, user_id);

            if let Err(_) = user {
                errors.push(FieldError::new(
                    "userId".to_owned(),
                    "User not logged in. (Please logout and login again)"
                        .to_owned(),
                ));
                return QuestionResponse::from_errors(errors);
            }

            let questions = services::question::delete_all_by_user_id(
                &mut conn,
                user_id,
            );

            match questions {
                Ok(_) => true,
                Err(_) => false
            }
        } else {
            errors.push(FieldError::new(
                "userId".to_owned(),
                "User not logged in.".to_owned(),
            ));
            QuestionResponse::from_errors(errors)
        }
    }

}
pub struct QuestionMutation;

#[juniper::graphql_object(Context = Context)]
impl QuestionMutation {
    fn create(ctx: &Context, text: String) -> QuestionResponse {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let text = trim_whitespace(&text);
        let mut errors = vec![];

        let user_id = ctx
            .session
            .get::<Uuid>("userId")
            .expect("Failed to get user id from session.");

        if let Some(user_id) = user_id {
            let user = services::user::get_by_id(&mut conn, user_id);

            if let Err(_) = user {
                errors.push(FieldError::new(
                    "userId".to_owned(),
                    "User not logged in. (Please logout and login again)"
                        .to_owned(),
                ));
                return QuestionResponse::from_errors(errors);
            }

            if !text.chars().all(|c| {
                c.is_alphabetic() || c == ' ' || c.is_ascii_punctuation()
            }) {
                errors.push(FieldError::new(
                    "question".to_owned(),
                    "Question contains invalid characters.".to_owned(),
                ));
                return QuestionResponse::from_errors(errors);
            }

            let question = services::question::get_by_text(&mut conn, &text);

            if let Ok(_) = question {
                errors.push(FieldError::new(
                    "question".to_owned(),
                    "Question already exists.".to_owned(),
                ));
                return QuestionResponse::from_errors(errors);
            }

            let question = services::question::create(
                &mut conn,
                QuestionInput { text, user_id },
            );

            match question {
                Ok(question) => QuestionResponse::from_question(question),
                Err(e) => {
                    errors.push(FieldError::new(
                        "question".to_owned(),
                        e.to_string(),
                    ));
                    QuestionResponse::from_errors(errors)
                }
            }
        } else {
            errors.push(FieldError::new(
                "userId".to_owned(),
                "User not logged in.".to_owned(),
            ));
            QuestionResponse::from_errors(errors)
        }
    }
}

fn trim_whitespace(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    s.split_whitespace().for_each(|w| {
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(w);
    });
    result
}
