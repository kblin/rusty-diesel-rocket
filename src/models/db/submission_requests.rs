use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::errors::MibigError;
use crate::schema::submission_requests;
use crate::schema::submission_requests::dsl::submission_requests as all_submission_requests;

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "submission_requests"]
pub struct SubmissionRequest {
    pub id: i64,
    pub user_id: String,
    pub compounds: Vec<String>,
    pub accession: String,
    pub start_nt: Option<i32>,
    pub end_nt: Option<i32>,
}

#[derive(Insertable, Debug)]
#[table_name = "submission_requests"]
pub struct NewSubmissionRequest {
    pub user_id: String,
    pub compounds: Vec<String>,
    pub accession: String,
    pub start_nt: Option<i32>,
    pub end_nt: Option<i32>,
}

impl SubmissionRequest {
    pub fn new(
        user_id: String,
        compounds: Vec<String>,
        accession: String,
        start_nt: Option<i32>,
        end_nt: Option<i32>,
        conn: &PgConnection,
    ) -> Result<SubmissionRequest, MibigError> {
        let new_submission = NewSubmissionRequest {
            user_id: user_id.into(),
            compounds: compounds.into(),
            accession: accession.into(),
            start_nt: start_nt,
            end_nt: end_nt,
        };
        let inserted = diesel::insert_into(submission_requests::table)
            .values(&new_submission)
            .get_result::<SubmissionRequest>(conn)?;
        Ok(inserted)
    }

    pub fn list(conn: &PgConnection) -> Result<Vec<SubmissionRequest>, MibigError> {
        let res = all_submission_requests
            .order(submission_requests::id)
            .load::<SubmissionRequest>(conn)?;
        Ok(res)
    }

    pub fn show(id: i64, conn: &PgConnection) -> Result<SubmissionRequest, MibigError> {
        let req = all_submission_requests.find(id).first(conn)?;
        Ok(req)
    }

    pub fn delete(id: i64, conn: &PgConnection) -> Result<(), MibigError> {
        SubmissionRequest::show(id, conn)?;
        diesel::delete(all_submission_requests.find(id)).execute(conn)?;
        Ok(())
    }
}
