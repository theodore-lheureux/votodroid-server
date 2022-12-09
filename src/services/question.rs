use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    models::question::{Question, QuestionInput},
    schema::{self, questions},
};
use schema::questions::dsl::*;

pub fn create(
    conn: &mut PgConnection,
    new_question: QuestionInput,
) -> QueryResult<Question> {
    diesel::insert_into(questions::table)
        .values(&new_question)
        .get_result(conn)
}

pub fn get_by_id(
    conn: &mut PgConnection,
    question_id: Uuid,
) -> QueryResult<Question> {
    questions.find(question_id).first(conn)
}

pub fn delete_all_by_user_id(
    conn: &mut PgConnection,
    userid: Uuid,
) -> QueryResult<usize> {
    diesel::delete(questions.filter(user_id.eq(userid))).execute(conn)
}

pub fn get_all(conn: &mut PgConnection) -> QueryResult<Vec<Question>> {
    questions.load(conn)
}

pub fn get_all_by_user_id(
    conn: &mut PgConnection,
    userid: Uuid,
) -> QueryResult<Vec<Question>> {
    questions.filter(user_id.eq(userid)).load(conn)
}

pub fn get_paginated(
    conn: &mut PgConnection,
    limit: i64,
    cursor: Option<Uuid>,
) -> QueryResult<Vec<Question>> {
    let mut query = questions.into_boxed();
    if let Some(cursor) = cursor {
        query = query.filter(id.lt(cursor));
    }
    query
        .order(id.desc())
        .limit(limit as i64)
        .load::<Question>(conn)
}