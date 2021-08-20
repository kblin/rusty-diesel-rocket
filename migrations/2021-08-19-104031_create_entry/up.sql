CREATE TABLE IF NOT EXISTS entries (
    entry_id text PRIMARY KEY,
    minimal bool NOT NULL,
    tax_id bigint NOT NULL,
    organism_name text NOT NULL,
    biosyn_class text[] NOT NULL,
    legacy_comment text
);

CREATE TABLE IF NOT EXISTS rel_entries_types (
    entry_id text REFERENCES entries ON DELETE CASCADE,
    bgc_type_id int REFERENCES bgc_types ON DELETE CASCADE,
    PRIMARY KEY (entry_id, bgc_type_id)
);