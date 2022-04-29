use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RiPP {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyclic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peptidases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precursor_genes: Option<Vec<RippPrecursor>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subclass: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RippPrecursor {
    #[serde(
        rename = "cleavage_recogn_site",
        skip_serializing_if = "Option::is_none"
    )]
    pub cleavage_recogn_sites: Option<Vec<String>>,
    #[serde(rename = "core_sequence")]
    pub core_sequences: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crosslinks: Option<Vec<RippCrosslink>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_sequence: Option<String>,
    pub gene_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_sequence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recogition_motif: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RippCrosslink {
    pub crosslink_type: String, // TODO: turn into an enum, also in schema?
    #[serde(rename = "first_AA", skip_serializing_if = "Option::is_none")]
    pub first_aa: Option<u64>,
    #[serde(rename = "last_AA", skip_serializing_if = "Option::is_none")]
    pub last_aa: Option<u64>,
}
