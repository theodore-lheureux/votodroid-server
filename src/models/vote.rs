use chrono::NaiveDateTime;
use diesel::prelude::*;
use juniper::GraphQLObject;
use uuid::Uuid;
use votodroid_server_derive::VotodroidResponseObject;

use crate::schema;

use super::types::FieldError;

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
    #[graphql(skip)]
    pub user_id: Uuid,
    /// The question for which the vote was created
    pub question_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = schema::votes)]
pub struct VoteInput {
    /// The vote's value
    pub value: i32,
    // The user who created the vote
    pub user_id: Uuid,
    /// The question for which the vote was created
    pub question_id: Uuid,
}

#[derive(GraphQLObject, VotodroidResponseObject)]
pub struct VoteResponse {
    pub vote: Option<Vote>,
    pub errors: Option<Vec<FieldError>>,
}