use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;
use votodroid_server_derive::VotodroidResponseObject;

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

#[derive(GraphQLObject, VotodroidResponseObject)]
pub struct QuestionResponse {
    pub question: Option<Question>,
    pub errors: Option<Vec<FieldError>>,
}

#[derive(GraphQLObject, VotodroidResponseObject)]
pub struct QuestionsResponse {
    pub questions: Option<Vec<Question>>,
    pub errors: Option<Vec<FieldError>>,
}
