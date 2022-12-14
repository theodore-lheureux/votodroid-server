use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::{
    context::Context,
    models::vote::{Vote, VoteInput, VoteResponse},
    services,
};

pub struct VoteQuery;

#[juniper::graphql_object(Context = Context)]
impl VoteQuery {
    fn get_avg_for_question(ctx: &Context, question_id: String) -> String {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let question_id = Uuid::parse_str(&question_id);

        if let Err(_e) = question_id {
            return "0.00".to_owned();
        }

        let avg = services::vote::avg_for_question(
            &mut conn,
            question_id.unwrap(),
        );

        match avg {
            Ok(avg) => format!("{:.2}", avg.unwrap_or(BigDecimal::from(0)).round(2)),
            Err(_e) => "0.00".to_owned(),
        }
    }
    fn get_all_for_question(ctx: &Context, question_id: String) -> Vec<Vote> {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let question_id = Uuid::parse_str(&question_id);

        if let Err(_e) = question_id {
            return Vec::new();
        }

        let votes = services::vote::get_all_by_question_id(
            &mut conn,
            question_id.unwrap(),
        );

        match votes {
            Ok(votes) => votes,
            Err(_e) => Vec::new(),
        }
    }
    fn get_stats_for_question(ctx: &Context, question_id: String) -> Vec<i32> {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let question_id = Uuid::parse_str(&question_id);

        if let Err(_e) = question_id {
            return vec![0, 0, 0, 0, 0, 0];
        }

        let mut result: Vec<i32> = vec![];

        for i in 0..6 {
            result.push(services::vote::count_for_question_with_value(&mut conn, question_id.clone().unwrap(), i).unwrap() as i32);
        }

        result
    }
}

pub struct VoteMutation;

#[juniper::graphql_object(Context = Context)]
impl VoteMutation {
    fn create(ctx: &Context, question_id: String, value: i32) -> VoteResponse {
        let mut conn = ctx
            .pool
            .get()
            .expect("Failed to get connection to database.");
        let user_id = ctx.session.get::<Uuid>("userId").unwrap();
        let question_id = Uuid::parse_str(&question_id);

        if let Err(e) = question_id {
            return VoteResponse::from_error(
                "questionId".to_owned(),
                e.to_string(),
            );
        }

        if value < 0 || value > 5 {
            return VoteResponse::from_error(
                "value".to_owned(),
                "Value must be from 0 to 5".to_owned(),
            );
        }

        if let Some(user_id) = user_id {
            let vote = services::vote::create(
                &mut conn,
                VoteInput {
                    value,
                    user_id,
                    question_id: question_id.unwrap(),
                },
            );

            match vote {
                Ok(vote) => VoteResponse::from_vote(vote),
                Err(e) => {
                    VoteResponse::from_error("userId".to_owned(), e.to_string())
                }
            }
        } else {
            VoteResponse::from_error(
                "userId".to_owned(),
                "User not logged in".to_owned(),
            )
        }
    }
}
