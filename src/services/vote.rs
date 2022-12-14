use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};
use uuid::Uuid;

use crate::schema::votes::dsl::*;
use crate::{
    models::vote::{Vote, VoteInput},
    schema::votes,
};

pub fn create(
    conn: &mut PgConnection,
    new_vote: VoteInput,
) -> QueryResult<Vote> {
    diesel::insert_into(votes::table)
        .values(&new_vote)
        .get_result(conn)
}

pub fn get_all_by_question_id(
    conn: &mut PgConnection,
    questionid: Uuid,
) -> QueryResult<Vec<Vote>> {
    votes.filter(question_id.eq(questionid)).load(conn)
}

pub fn get_by_user_id_and_question_id(
    conn: &mut PgConnection,
    userid: Uuid,
    questionid: Uuid,
) -> QueryResult<Vote> {
    votes
        .filter(user_id.eq(userid))
        .filter(question_id.eq(questionid))
        .first(conn)
}

pub fn update(
    conn: &mut PgConnection,
    voteid: Uuid,
    new_value: i32,
) -> QueryResult<Vote> {
    diesel::update(votes.find(voteid))
        .set((value.eq(new_value), updated_at.eq(diesel::dsl::now)))
        .get_result(conn)
}

pub fn avg_for_question(
    conn: &mut PgConnection,
    questionid: Uuid,
) -> QueryResult<Option<BigDecimal>> {
    votes
        .filter(question_id.eq(questionid))
        .select(diesel::dsl::avg(value))
        .first(conn)
}

pub fn count_for_question_with_value(
    conn: &mut PgConnection,
    questionid: Uuid,
    votevalue: i32,
) -> QueryResult<i64> {
    votes
        .filter(question_id.eq(questionid))
        .filter(value.eq(votevalue))
        .count()
        .get_result(conn)
}
