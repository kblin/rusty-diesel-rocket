use std::collections::HashMap;

use serde_json;
use structopt::clap::arg_enum;
use structopt::StructOpt;

use crate::errors::MibigError;
use crate::utils;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Debug, StructOpt)]
pub struct RepoOpts {
    #[structopt(subcommand)]
    cmd: Option<RepoSubcommand>,
}

#[derive(Debug, StructOpt)]
enum RepoSubcommand {
    #[structopt(name = "list", about = "List repository entries")]
    List(RepoListOpts),

    #[structopt(name = "import", about = "Import a new entry")]
    Import(RepoImportOpts),
}

#[derive(Debug, StructOpt)]
pub struct RepoListOpts {
    #[structopt(short, long, help = "Status of the entry", possible_values = &RepoListEntryStatus::variants(), case_insensitive = true, default_value = "Published")]
    status: RepoListEntryStatus,
}

structopt::clap::arg_enum! {
    #[derive(Debug)]
    enum RepoListEntryStatus {
        Published,
        Retired,
        Embargoed,
        Reserved,
    }
}

#[derive(Debug, StructOpt)]
pub struct RepoImportOpts {
    #[structopt(
        parse(from_os_str),
        help = "File or directory containing MIBiG JSON data"
    )]
    input: std::path::PathBuf,
    #[structopt(short, long, help = "Only run json parsing")]
    json_only: bool,
}

pub fn repo(cfg: RepoOpts) {
    let conn = utils::db::establish_connection();

    match cfg.cmd {
        Some(cmd) => match cmd {
            RepoSubcommand::List(opts) => repo_list(opts.status),
            RepoSubcommand::Import(opts) => repo_import(opts.input, conn, opts.json_only),
        },
        None => repo_list(RepoListEntryStatus::Published),
    }
}

use crate::models;

fn repo_import(input: std::path::PathBuf, conn: PgConnection, json_only: bool) {
    if input.is_file() {
        import_file(input, None, &conn, json_only);
        return;
    }

    let mut taxon_cache: HashMap<i64, utils::taxa::NcbiTaxEntry> = HashMap::new();

    let mut entries = std::fs::read_dir(input)
        .expect("failed to read input")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();

    entries.sort();

    for path in entries {
        if "json" != path.extension().unwrap() {
            continue;
        }
        eprintln!("Loading {:?}", path);
        import_file(path, Some(&mut taxon_cache), &conn, json_only);
    }
}

fn import_file(
    infile: std::path::PathBuf,
    cache: Option<&mut HashMap<i64, utils::taxa::NcbiTaxEntry>>,
    conn: &PgConnection,
    json_only: bool,
) {
    let content = std::fs::read_to_string(&infile).expect("could not read file");
    let entry: models::schema::Entry = serde_json::from_str(&content).unwrap();

    if !json_only {
        insert_into_db(&entry, cache, conn);
    }

    let return_value = serde_json::to_string_pretty(&entry).expect("failed to serialize");
    println!("{}", return_value)
}

fn insert_into_db(
    entry: &models::schema::Entry,
    cache: Option<&mut HashMap<i64, utils::taxa::NcbiTaxEntry>>,
    conn: &PgConnection,
) {
    let tax_id = get_or_create_taxid(
        entry.cluster.organism_name.as_str(),
        entry.cluster.ncbi_tax_id,
        cache,
        conn,
    )
    .expect("Error getting tax_id");

    let new_entry = models::db::NewEntry {
        id: entry.cluster.mibig_accession.to_owned(),
        biosyn_class: entry
            .cluster
            .biosyn_class
            .to_vec()
            .iter()
            .map(|cls| {
                serde_json::to_string(&cls)
                    .expect("failed to convert biosyn_class")
                    .trim_matches('"')
                    .to_string()
            })
            .collect(),
        minimal: entry.cluster.minimal,
        organism_name: entry.cluster.organism_name.to_string(),
        tax_id: tax_id,
        legacy_comment: entry.comments.to_owned(),
    };

    let created_entry: models::db::Entry = diesel::insert_into(crate::schema::entries::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving entry");

    eprintln!("{:?}", created_entry);
}

fn get_or_create_taxid<'a>(
    organism_name: &'a str,
    ncbi_tax_id: i64,
    cache: Option<&'a mut HashMap<i64, utils::taxa::NcbiTaxEntry>>,
    conn: &'a PgConnection,
) -> Result<i64, MibigError> {
    use crate::schema::taxa::dsl::*;

    let loaded_taxid: i64 = match taxa
        .filter(ncbi_taxid.eq(ncbi_tax_id))
        .filter(name.eq(organism_name))
        .select(tax_id)
        .first(conn)
    {
        Ok(number) => number,
        Err(_) => {
            let tax_info = utils::taxa::entry_for_taxid(ncbi_tax_id, cache)?;
            let new_tax_entry = models::db::NewTaxon {
                ncbi_taxid: ncbi_tax_id,
                superkingdom: tax_info.superkingdom,
                kingdom: tax_info.kingdom,
                phylum: tax_info.phylum,
                class: tax_info.class,
                taxonomic_order: tax_info.order,
                family: tax_info.family,
                genus: tax_info.genus,
                species: tax_info.species,
                name: organism_name.to_string(),
            };
            let created_entry: models::db::Taxon = diesel::insert_into(crate::schema::taxa::table)
                .values(&new_tax_entry)
                .get_result(conn)
                .expect("Error saving new tax entry");

            created_entry.tax_id
        }
    };
    Ok(loaded_taxid)
}

fn repo_list(status: RepoListEntryStatus) {
    println!("Listing stuff on level {:?}", status)
}
