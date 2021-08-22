table! {
    bgc_types (bgc_type_id) {
        bgc_type_id -> Int4,
        term -> Text,
        name -> Text,
        description -> Text,
        parent_id -> Nullable<Int4>,
        safe_class -> Text,
    }
}

table! {
    entries (entry_id) {
        entry_id -> Text,
        minimal -> Bool,
        tax_id -> Int8,
        organism_name -> Text,
        biosyn_class -> Array<Text>,
        legacy_comment -> Nullable<Text>,
    }
}

table! {
    rel_entries_types (entry_id, bgc_type_id) {
        entry_id -> Text,
        bgc_type_id -> Int4,
    }
}

table! {
    taxa (tax_id) {
        tax_id -> Int8,
        ncbi_taxid -> Int8,
        superkingdom -> Text,
        kingdom -> Text,
        phylum -> Text,
        class -> Text,
        taxonomic_order -> Text,
        family -> Text,
        genus -> Text,
        species -> Text,
        name -> Text,
    }
}

joinable!(entries -> taxa (tax_id));
joinable!(rel_entries_types -> bgc_types (bgc_type_id));
joinable!(rel_entries_types -> entries (entry_id));

allow_tables_to_appear_in_same_query!(
    bgc_types,
    entries,
    rel_entries_types,
    taxa,
);
