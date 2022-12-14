use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::schema;

use super::types::FieldError;

#[derive(Clone, Queryable, GraphQLObject)]
///A question
pub struct Question {
    /// The question's id (UUID)
    pub id: Uuid,
    /// The question's text
    pub text: String,
    /// The date and time the question was created
    pub created_at: NaiveDateTime,
    /// The date and time the question was last updated
    pub updated_at: NaiveDateTime,
    /// The user who created the question
    pub user_id: Uuid,
}

#[derive(GraphQLInputObject, Insertable)]
#[diesel(table_name = schema::questions)]
pub struct QuestionInput {
    /// The question's text
    pub text: String,
    // The user who created the question
    pub user_id: Uuid,
}

#[derive(GraphQLObject)]
pub struct QuestionResponse {
    pub question: Option<Question>,
    pub errors: Option<Vec<FieldError>>,
}

impl QuestionResponse {
    pub fn from_question(question: Question) -> QuestionResponse {
        QuestionResponse {
            question: Some(question),
            errors: None,
        }
    }
    pub fn from_error(field: String, message: String) -> QuestionResponse {
        QuestionResponse {
            question: None,
            errors: Some(vec![FieldError::new(field, message)]),
        }
    }
}

#[derive(GraphQLObject)]
pub struct QuestionsResponse {
    pub questions: Option<Vec<Question>>,
    pub errors: Option<Vec<FieldError>>,
}

impl QuestionsResponse {
    pub fn from_questions(questions: Vec<Question>) -> QuestionsResponse {
        QuestionsResponse {
            questions: Some(questions),
            errors: None,
        }
    }
    pub fn from_error(field: String, message: String) -> QuestionsResponse {
        QuestionsResponse {
            questions: None,
            errors: Some(vec![FieldError::new(field, message)]),
        }
    }
}
