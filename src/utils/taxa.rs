use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use dotenv::dotenv;

#[derive(Debug)]
pub struct NcbiTaxEntry {
    tax_id: u64,
    name: String,
    species: String,
    genus: String,
    family: String,
    order: String,
    class: String,
    phylum: String,
    kingdom: String,
    superkingdom: String,
}

pub struct NcbiTaxEntryError<'a> {
    message: &'a str,
}

impl fmt::Display for NcbiTaxEntryError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::io::Error> for NcbiTaxEntryError<'_> {
    fn from(item: std::io::Error) -> Self {
        NcbiTaxEntryError {
            message: format!("{}", item),
        }
    }
}

pub fn entry_for_taxid(tax_id: u64) -> Result<NcbiTaxEntry, NcbiTaxEntryError<'static>> {
    dotenv().ok();

    let dump_file_name = env::var("LINEAGE_DUMP")
        .expect("LINEAGE_DUMP must point at NCBI taxonomy lineage dump file");

    if let Ok(lines) = read_lines(dump_file_name) {
        for line_option in lines {
            if let Ok(line) = line_option {
                let mut parts: Vec<&str> = line.trim().splitn(11, "|").collect();
                for (i, part) in parts.iter().copied().enumerate() {
                    let mut trimmed = part.trim();
                    if trimmed == "" {
                        trimmed = "Unknown"
                    }
                    parts[i] = trimmed;
                }

                let curr_tax_id = match parts[0].parse::<u64>() {
                    Ok(i) => i,
                    Err(_) => {
                        return Err(NcbiTaxEntryError {
                            message: "couldn't parse taxid",
                        })
                    }
                };

                if curr_tax_id != tax_id {
                    continue;
                };

                return Ok(NcbiTaxEntry {
                    tax_id: tax_id,
                    name: String::from(parts[1]),
                    species: String::from(parts[2]),
                    genus: String::from(parts[3]),
                    family: String::from(parts[4]),
                    order: String::from(parts[5]),
                    class: String::from(parts[6]),
                    phylum: String::from(parts[7]),
                    kingdom: String::from(parts[8]),
                    superkingdom: String::from(parts[9]),
                });
            }
        }
    };
    Err(NcbiTaxEntryError {
        message: "No entry found for taxid",
    })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
