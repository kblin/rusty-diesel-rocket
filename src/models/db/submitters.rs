use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::errors::MibigError;
use crate::schema::submitters;
use crate::schema::submitters::dsl::submitters as all_submitters;
use crate::utils::check_password;
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
    pub fn get_id_by_email(email: CiString, conn: &PgConnection) -> Result<String, MibigError> {
        let id = all_submitters
            .filter(submitters::email.eq(email))
            .select(submitters::user_id)
            .first(conn)?;
        Ok(id)
    }

    pub fn show(id: &String, conn: &PgConnection) -> Result<Submitter, MibigError> {
        let submitter = all_submitters.find(id).first(conn)?;
        Ok(submitter)
    }

    pub fn all(conn: &PgConnection) -> Result<Vec<Submitter>, MibigError> {
        let res = all_submitters
            .order(submitters::user_id.desc())
            .load::<Submitter>(conn)?;
        Ok(res)
    }

    pub fn insert(submitter: Submitter, conn: &PgConnection) -> Result<(), MibigError> {
        diesel::insert_into(submitters::table)
            .values(&submitter)
            .execute(conn)?;
        Ok(())
    }

    pub fn update(
        id: &String,
        submitter: Submitter,
        conn: &PgConnection,
    ) -> Result<i32, MibigError> {
        use crate::schema::submitters::dsl::{
            active, call_name, email, gdpr_consent, institution, is_public, name, password_hash,
            version,
        };
        let updated = diesel::update(
            all_submitters
                .find(id)
                .filter(version.eq(submitter.version)),
        )
        .set((
            email.eq(submitter.email),
            name.eq(submitter.name),
            call_name.eq(submitter.call_name),
            institution.eq(submitter.institution),
            password_hash.eq(submitter.password_hash),
            is_public.eq(submitter.is_public),
            gdpr_consent.eq(submitter.gdpr_consent),
            active.eq(submitter.active),
            version.eq(submitter.version + 1),
        ))
        .get_result::<Submitter>(conn)?;
        Ok(updated.version)
    }

    pub fn delete(id: &String, conn: &PgConnection) -> Result<(), MibigError> {
        Submitter::show(id, conn)?;
        diesel::delete(all_submitters.find(id)).execute(conn)?;
        Ok(())
    }

    pub fn check_password(&self, password: String) -> Result<bool, MibigError> {
        match &self.password_hash {
            None => Ok(false),
            Some(hash) => check_password(password, hash.clone().to_string()),
        }
    }
}
