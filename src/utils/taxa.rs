use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct NcbiTaxEntry {
    pub tax_id: i64,
    pub name: String,
    pub species: String,
    pub genus: String,
    pub family: String,
    pub order: String,
    pub class: String,
    pub phylum: String,
    pub kingdom: String,
    pub superkingdom: String,
}

use crate::errors::MibigError;

pub fn entry_for_taxid(
    tax_id: i64,
    cache: Option<&mut HashMap<i64, NcbiTaxEntry>>,
) -> Result<NcbiTaxEntry, MibigError> {
    dotenv().ok();

    let dump_file_name = env::var("LINEAGE_DUMP")
        .expect("LINEAGE_DUMP must point at NCBI taxonomy lineage dump file");

    match cache {
        None => return get_taxid_without_cache(dump_file_name, tax_id),
        Some(cache) => {
            if cache.is_empty() {
                eprint!("Populating taxon cache");
                let start = Instant::now();
                populate_cache(dump_file_name, cache)?;
                let duration = start.elapsed();
                eprintln!("done, took {} s", duration.as_secs());
            }
            if let Some(entry) = cache.get(&tax_id) {
                return Ok(entry.clone());
            } else {
                let new_id = detect_merged_id(tax_id)?;
                if let Some(entry) = cache.get(&new_id) {
                    return Ok(entry.clone());
                }
            }
        }
    }

    Err(MibigError::InvalidTaxID(format!(
        "Invalid taxid {}",
        tax_id
    )))
}

fn get_taxid_without_cache(file_name: String, tax_id: i64) -> Result<NcbiTaxEntry, MibigError> {
    if let Ok(lines) = read_lines(file_name) {
        for line_option in lines {
            if let Ok(line) = line_option {
                let parts: Vec<String> = line
                    .trim()
                    .splitn(11, "|")
                    .map(|part| match part.trim() {
                        "" => "Unknown".to_string(),
                        part => part.to_string(),
                    })
                    .collect();

                let curr_tax_id = match parts[0].parse::<i64>() {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(MibigError::InvalidTaxID(format!(
                            "couldn't parse taxid {}",
                            parts[0]
                        )))
                    }
                };

                if curr_tax_id != tax_id {
                    continue;
                };

                return Ok(NcbiTaxEntry {
                    tax_id: tax_id,
                    name: parts[1].to_owned(),
                    species: parts[2]
                        .split_whitespace()
                        .next_back()
                        .unwrap_or(parts[2].as_str())
                        .to_owned(),
                    genus: parts[3].to_owned(),
                    family: parts[4].to_owned(),
                    order: parts[5].to_owned(),
                    class: parts[6].to_owned(),
                    phylum: parts[7].to_owned(),
                    kingdom: parts[8].to_owned(),
                    superkingdom: parts[9].to_owned(),
                });
            }
        }
    };
    Err(MibigError::InvalidTaxID(format!(
        "No entry found for taxid {}",
        tax_id
    )))
}

fn populate_cache(
    file_name: String,
    cache: &mut HashMap<i64, NcbiTaxEntry>,
) -> Result<(), MibigError> {
    let mut counter = 0;

    if let Ok(lines) = read_lines(file_name) {
        for line_option in lines {
            if counter % 100_000 == 0 {
                eprint!(".");
            }
            counter += 1;
            if let Ok(line) = line_option {
                let parts: Vec<String> = line
                    .trim()
                    .splitn(11, "|")
                    .map(|part| match part.trim() {
                        "" => "Unknown".to_string(),
                        part => part.to_string(),
                    })
                    .collect();

                let tax_id = match parts[0].parse::<i64>() {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(MibigError::InvalidTaxID(format!(
                            "couldn't parse taxid {}",
                            parts[0]
                        )))
                    }
                };

                let entry = NcbiTaxEntry {
                    tax_id: tax_id,
                    name: parts[1].to_owned(),
                    species: parts[2]
                        .split_whitespace()
                        .next_back()
                        .unwrap_or(parts[2].as_str())
                        .to_owned(),
                    genus: parts[3].to_owned(),
                    family: parts[4].to_owned(),
                    order: parts[5].to_owned(),
                    class: parts[6].to_owned(),
                    phylum: parts[7].to_owned(),
                    kingdom: parts[8].to_owned(),
                    superkingdom: parts[9].to_owned(),
                };
                cache.insert(tax_id, entry.to_owned());
            }
        }
    };
    Ok(())
}

fn detect_merged_id(tax_id: i64) -> Result<i64, MibigError> {
    dotenv().ok();

    let merged_file_name =
        env::var("MERGED_DUMP").expect("MERGED_DUMP must point at NCBI taxonomy merged dump file");

    if let Ok(lines) = read_lines(merged_file_name) {
        for line_option in lines {
            if let Ok(line) = line_option {
                let parts: Vec<String> = line
                    .trim()
                    .splitn(3, "|")
                    .map(|part| part.trim().to_string())
                    .collect();

                let old_id = match parts[0].parse::<i64>() {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(MibigError::InvalidTaxID(format!(
                            "couldn't parse old taxid {}",
                            parts[0]
                        )))
                    }
                };

                if old_id != tax_id {
                    continue;
                };

                let new_id = match parts[1].parse::<i64>() {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(MibigError::InvalidTaxID(format!(
                            "couldn't parse new taxid {}",
                            parts[1]
                        )))
                    }
                };
                return Ok(new_id);
            }
        }
    };
    Err(MibigError::InvalidTaxID(format!(
        "No entry found for taxid {}",
        tax_id
    )))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
