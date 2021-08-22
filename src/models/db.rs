use crate::schema::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "taxa"]
#[primary_key(tax_id)]
pub struct Taxon {
    pub tax_id: i64,
    pub ncbi_taxid: i64,
    pub superkingdom: String,
    pub kingdom: String,
    pub phylum: String,
    pub class: String,
    pub taxonomic_order: String,
    pub family: String,
    pub genus: String,
    pub species: String,
    pub name: String,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "taxa"]
pub struct NewTaxon {
    pub ncbi_taxid: i64,
    pub superkingdom: String,
    pub kingdom: String,
    pub phylum: String,
    pub class: String,
    pub taxonomic_order: String,
    pub family: String,
    pub genus: String,
    pub species: String,
    pub name: String,
}

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
