use mibig_taxa::{NcbiTaxEntry, TaxonCache};

use crate::errors::MibigError;

pub fn entry_for_taxid(tax_id: i64, cache: &TaxonCache) -> Result<NcbiTaxEntry, MibigError> {
    if let Some(entry) = cache.mappings.get(&tax_id) {
        return Ok(entry.clone());
    } else {
        if let Some(new_id) = cache.deprecated_ids.get(&tax_id) {
            if let Some(entry) = cache.mappings.get(&new_id) {
                return Ok(entry.clone());
            }
        }
    }

    Err(MibigError::InvalidTaxID(format!(
        "Invalid taxid {}",
        tax_id
    )))
}
