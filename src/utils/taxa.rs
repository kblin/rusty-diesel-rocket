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
