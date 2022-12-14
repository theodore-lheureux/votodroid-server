use crate::{
    context::Context,
    graphql::{
        question_resolver::QuestionQuery,
        user_resolver::{UserMutation, UserQuery},
    },
};

use juniper::graphql_object;

use self::question_resolver::QuestionMutation;

mod question_resolver;
mod user_resolver;
mod vote_resolver;

pub struct QueryRoot;

#[graphql_object(Context = Context,
    description = "Query Root",)]
impl QueryRoot {
    fn api_version() -> &'static str {
        "1.0"
    }
    fn users(&self) -> UserQuery {
        UserQuery
    }
    fn questions(&self) -> QuestionQuery {
        QuestionQuery
    }
    fn votes(&self) -> vote_resolver::VoteQuery {
        vote_resolver::VoteQuery
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context,
    description = "Mutation Root",)]
impl MutationRoot {
    fn users(&self) -> UserMutation {
        UserMutation
    }
    fn questions(&self) -> QuestionMutation {
        QuestionMutation
    }
    fn votes(&self) -> vote_resolver::VoteMutation {
        vote_resolver::VoteMutation
    }
}
