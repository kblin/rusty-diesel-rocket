use crate::schema::*;

pub mod submitters;
pub mod taxa;

pub use taxa::*;

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "entries"]
pub struct Entry {
    pub id: String,
    pub minimal: bool,
    pub tax_id: i64,
    pub organism_name: String,
    pub biosyn_class: Vec<String>,
    pub legacy_comment: Option<String>,
}

#[derive(Insertable)]
#[table_name = "entries"]
pub struct NewEntry {
    pub id: String,
    pub minimal: bool,
    pub tax_id: i64,
    pub organism_name: String,
    pub biosyn_class: Vec<String>,
    pub legacy_comment: Option<String>,
}
