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

pub fn get_by_id(conn: &mut PgConnection, voteid: Uuid) -> QueryResult<Vote> {
    votes.find(voteid).first(conn)
}

pub fn get_all(conn: &mut PgConnection) -> QueryResult<Vec<Vote>> {
    votes.load(conn)
}

pub fn get_all_by_user_id(
    conn: &mut PgConnection,
    userid: Uuid,
) -> QueryResult<Vec<Vote>> {
    votes.filter(user_id.eq(userid)).load(conn)
}

pub fn get_all_by_question_id(
    conn: &mut PgConnection,
    questionid: Uuid,
) -> QueryResult<Vec<Vote>> {
    votes.filter(question_id.eq(questionid)).load(conn)
}

pub fn get_all_by_user_id_and_question_id(
    conn: &mut PgConnection,
    userid: Uuid,
    questionid: Uuid,
) -> QueryResult<Vec<Vote>> {
    votes
        .filter(user_id.eq(userid))
        .filter(question_id.eq(questionid))
        .load(conn)
}

pub fn delete_all_by_user_id(
    conn: &mut PgConnection,
    userid: Uuid,
) -> QueryResult<usize> {
    diesel::delete(votes.filter(user_id.eq(userid))).execute(conn)
}

pub fn delete_all_by_question_id(
    conn: &mut PgConnection,
    questionid: Uuid,
) -> QueryResult<usize> {
    diesel::delete(votes.filter(question_id.eq(questionid))).execute(conn)
}

pub fn delete_all_by_user_id_and_question_id(
    conn: &mut PgConnection,
    userid: Uuid,
    questionid: Uuid,
) -> QueryResult<usize> {
    diesel::delete(
        votes
            .filter(user_id.eq(userid))
            .filter(question_id.eq(questionid)),
    )
    .execute(conn)
}

pub fn delete_by_id(
    conn: &mut PgConnection,
    voteid: Uuid,
) -> QueryResult<usize> {
    diesel::delete(votes.filter(id.eq(voteid))).execute(conn)
}

pub fn delete_all(conn: &mut PgConnection) -> QueryResult<usize> {
    diesel::delete(votes).execute(conn)
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
