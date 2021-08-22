CREATE TABLE IF NOT EXISTS taxa (
    tax_id	bigserial PRIMARY KEY,
    ncbi_taxid	bigint NOT NULL,
    superkingdom	text NOT NULL,
    kingdom	text NOT NULL,
    phylum	text NOT NULL,
    class	text NOT NULL,
    taxonomic_order	text NOT NULL,
    family	text NOT NULL,
    genus	text NOT NULL,
    species	text NOT NULL,
    name	text UNIQUE NOT NULL
);
