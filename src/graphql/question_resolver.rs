use juniper::FieldResult;
use uuid::Uuid;

use crate::{
    context::Context,
    models::{
        question::{QuestionInput, QuestionResponse, QuestionsResponse},
        types::FieldError,
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

        if let Err(e) = question_id {
            return QuestionResponse::from_error(
                "questionId".to_owned(),
                e.to_string(),
            );
        }

        let question = get_by_id(&mut conn, question_id.unwrap());

        match question {
            Ok(question) => QuestionResponse::from_question(question),
            Err(e) => QuestionResponse::from_error(
                "questionId".to_owned(),
                e.to_string(),
            ),
        }
    }

    fn get_paginated(
        ctx: &Context,
        limit: Option<i32>,
        cursor: Option<String>,
    ) -> QuestionsResponse {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let limit = limit.unwrap_or(20);

        let cursor = match cursor {
            Some(cursor) => {
                let cursor = Uuid::parse_str(&cursor);

                if let Err(e) = cursor {
                    return QuestionsResponse::from_error(
                        "cursor".to_owned(),
                        e.to_string(),
                    );
                }

                Some(cursor.unwrap())
            }
            None => None,
        };

        if let Some(cursor) = cursor {
            let question = get_by_id(&mut conn, cursor);

            if question.is_err() {
                return QuestionsResponse::from_error(
                    "cursor".to_owned(),
                    "No question found with corresponding Id.".to_owned(),
                );
            }
        }

        let questions =
            services::question::get_paginated(&mut conn, limit, cursor);

        QuestionsResponse::from_questions(
            questions.expect("Failed to get questions."),
        )
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

        let user_id = ctx
            .session
            .get::<Uuid>("userId")
            .expect("Failed to get user id from session.");

        if let Some(user_id) = user_id {
            let user = services::user::get_by_id(&mut conn, user_id);

            if user.is_err() {
                return QuestionResponse::from_error(
                    "userId".to_owned(),
                    "User not logged in. (Please logout and login again)"
                        .to_owned(),
                );
            }

            if text.len() < 4 {
                return QuestionResponse::from_error(
                    "question".to_owned(),
                    "Question must be at least 4 characters long."
                        .to_owned(),
                );
            }

            if !text.chars().all(|c| {
                c.is_alphabetic() || c == ' ' || c.is_ascii_punctuation()
            }) {
                return QuestionResponse::from_error(
                    "question".to_owned(),
                    "Question contains invalid characters.".to_owned(),
                );
            }

            let question = services::question::get_by_text(&mut conn, &text);

            if question.is_ok() {
                return QuestionResponse::from_error(
                    "question".to_owned(),
                    "Question already exists.".to_owned(),
                );
            }

            let question = services::question::create(
                &mut conn,
                QuestionInput { text, user_id },
            );

            match question {
                Ok(question) => QuestionResponse::from_question(question),
                Err(e) => QuestionResponse::from_error(
                    "question".to_owned(),
                    e.to_string(),
                ),
            }
        } else {
            QuestionResponse::from_error(
                "userId".to_owned(),
                "User not logged in.".to_owned(),
            )
        }
    }

    fn delete_all_by_user(ctx: &Context) -> FieldResult<bool> {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");

        let user_id = ctx
            .session
            .get::<Uuid>("userId")
            .expect("Failed to get user id from session.");

        if let Some(user_id) = user_id {
            let user = services::user::get_by_id(&mut conn, user_id);

            if user.is_err() {
                return Err(juniper::FieldError::from(
                    "User not logged in. (Please logout and login again)",
                ));
            }

            let result =
                services::question::delete_all_by_user_id(&mut conn, user_id);

            if let Err(e) = result {
                return Err(juniper::FieldError::from(e.to_string()));
            }

            return Ok(true);
        }
        Err(juniper::FieldError::from("User not logged in."))
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
