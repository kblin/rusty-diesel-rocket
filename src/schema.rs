table! {
    use diesel::sql_types::*;
    use crate::utils::typedefs::sql_types::*;

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
    use diesel::sql_types::*;
    use crate::utils::typedefs::sql_types::*;

    entries (id) {
        id -> Text,
        minimal -> Bool,
        tax_id -> Int8,
        organism_name -> Text,
        biosyn_class -> Array<Text>,
        legacy_comment -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::utils::typedefs::sql_types::*;

    rel_entries_types (entry_id, bgc_type_id) {
        entry_id -> Text,
        bgc_type_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::utils::typedefs::sql_types::*;

    rel_submitters_roles (user_id, role_id) {
        user_id -> Text,
        role_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::utils::typedefs::sql_types::*;

    roles (role_id) {
        role_id -> Int4,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::utils::typedefs::sql_types::*;

    submission_requests (id) {
        id -> Int8,
        user_id -> Text,
        compounds -> Array<Text>,
        accession -> Text,
        start_nt -> Nullable<Int4>,
        end_nt -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::utils::typedefs::sql_types::*;

    submitters (user_id) {
        user_id -> Text,
        email -> Citext,
        name -> Nullable<Text>,
        call_name -> Nullable<Text>,
        institution -> Nullable<Text>,
        password_hash -> Nullable<Text>,
        is_public -> Bool,
        gdpr_consent -> Bool,
        active -> Bool,
        version -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::utils::typedefs::sql_types::*;

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

table! {
    use diesel::sql_types::*;
    use crate::utils::typedefs::sql_types::*;

    tokens (hash) {
        hash -> Bytea,
        user_id -> Text,
        expiry -> Timestamptz,
        scope -> Text,
    }
}

joinable!(entries -> taxa (tax_id));
joinable!(rel_entries_types -> bgc_types (bgc_type_id));
joinable!(rel_entries_types -> entries (entry_id));
joinable!(rel_submitters_roles -> roles (role_id));
joinable!(rel_submitters_roles -> submitters (user_id));
joinable!(submission_requests -> submitters (user_id));
joinable!(tokens -> submitters (user_id));

allow_tables_to_appear_in_same_query!(
    bgc_types,
    entries,
    rel_entries_types,
    rel_submitters_roles,
    roles,
    submission_requests,
    submitters,
    taxa,
    tokens,
);
