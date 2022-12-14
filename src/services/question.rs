use diesel::{dsl::sql, prelude::*, sql_types::Text};
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
    question_uuid: Uuid,
) -> QueryResult<Question> {
    questions.find(question_uuid).first(conn)
}

pub fn get_by_text(
    conn: &mut PgConnection,
    question_text: &String,
) -> QueryResult<Question> {
    questions.filter(text.like(question_text)).first(conn)
}

pub fn delete_all_by_user_id(
    conn: &mut PgConnection,
    userid: Uuid,
) -> QueryResult<usize> {
    diesel::delete(questions.filter(user_id.eq(userid))).execute(conn)
}

pub fn get_paginated(
    conn: &mut PgConnection,
    limit: i32,
    cursor: Option<Uuid>,
) -> QueryResult<Vec<Question>> {
    let mut query = questions.into_boxed();
    if let Some(cursor) = cursor {
        query = query.filter(id.lt(cursor));
    }
    query
        .order_by(sql::<Text>("(SELECT COUNT(value) FROM votes WHERE votes.question_id = questions.id) DESC"))
        .limit(limit as i64)
        .load(conn)
}
