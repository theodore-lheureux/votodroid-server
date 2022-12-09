use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::{GraphQLInputObject, GraphQLObject};
use uuid::Uuid;

use crate::schema;

use super::field_error::FieldError;

#[derive(Clone, Queryable, GraphQLObject)]
///A vote
pub struct Vote {
    /// The vote's id (UUID)
    pub id: Uuid,
    /// The vote's value
    pub value: i32,
    /// The date and time the vote was created
    pub created_at: NaiveDateTime,
    /// The date and time the vote was last updated
    pub updated_at: NaiveDateTime,
    /// The user who created the vote
    pub user_id: Uuid,
    /// The question for which the vote was created
    pub question_id: Uuid,
}

#[derive(GraphQLInputObject, Insertable)]
#[diesel(table_name = schema::votes)]
pub struct VoteInput {
    /// The question's text
    pub value: i32,
    // The user who created the question
    pub user_id: Uuid,
    /// The question for which the vote was created
    pub question_id: Uuid,
}

#[derive(GraphQLObject)]
pub struct VoteResponse {
    pub vote: Option<Vote>,
    pub errors: Option<Vec<FieldError>>,
}

impl VoteResponse {
    pub fn from_vote(vote: Vote) -> VoteResponse {
        VoteResponse {
            vote: Some(vote),
            errors: None,
        }
    }
    pub fn from_errors(errors: Vec<FieldError>) -> VoteResponse {
        VoteResponse {
            vote: None,
            errors: Some(errors),
        }
    }
}
