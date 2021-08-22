use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use dotenv::dotenv;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct NcbiTaxEntryError {
    message: String,
}

impl fmt::Display for NcbiTaxEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for NcbiTaxEntryError {
    fn from(item: std::io::Error) -> Self {
        NcbiTaxEntryError {
            message: format!("{}", item),
        }
    }
}

pub fn entry_for_taxid(tax_id: i64) -> Result<NcbiTaxEntry, NcbiTaxEntryError> {
    dotenv().ok();

    let dump_file_name = env::var("LINEAGE_DUMP")
        .expect("LINEAGE_DUMP must point at NCBI taxonomy lineage dump file");

    if let Ok(lines) = read_lines(dump_file_name) {
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
                        return Err(NcbiTaxEntryError {
                            message: "couldn't parse taxid".to_owned(),
                        })
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
    Err(NcbiTaxEntryError {
        message: "No entry found for taxid".to_owned(),
    })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
