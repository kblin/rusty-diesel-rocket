use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::submitters;
use crate::schema::submitters::dsl::submitters as all_submitters;
use crate::utils::typedefs::types::*;

#[derive(Insertable, Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "submitters"]
#[primary_key(user_id)]
pub struct Submitter {
    pub user_id: String,
    pub email: CiString,
    pub name: Option<String>,
    pub call_name: Option<String>,
    pub institution: Option<String>,
    pub password_hash: Option<String>,
    pub is_public: bool,
    pub gdpr_consent: bool,
    pub active: bool,
    pub version: i32,
}

impl Submitter {
    pub fn show(id: String, conn: &PgConnection) -> Vec<Submitter> {
        all_submitters
            .find(id)
            .load::<Submitter>(conn)
            .expect("Error loading submitter")
    }

    pub fn all(conn: &PgConnection) -> Vec<Submitter> {
        all_submitters
            .order(submitters::user_id.desc())
            .load::<Submitter>(conn)
            .expect("Error submitters")
    }

    pub fn insert(submitter: Submitter, conn: &PgConnection) -> bool {
        diesel::insert_into(submitters::table)
            .values(&submitter)
            .execute(conn)
            .is_ok()
    }
}
