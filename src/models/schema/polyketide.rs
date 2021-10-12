use serde::{Deserialize, Serialize};

use super::nrp::{NonCanonicalModule, SubstrateSpecificityEvidence, Thioesterase};

#[derive(Debug, Deserialize, Serialize)]
pub struct Polyketide {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyclases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyclic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ketide_length: Option<i64>, // TODO: Turn into a u64 once spec is fixed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_type: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starter_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subclasses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthases: Option<Vec<PksSynthase>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PksSynthase {
    pub genes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterative: Option<PksIterative>,
    // TODO: Modules
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pufa_modification_domains: Option<Vec<String>>,
    #[serde(rename = "subclass", skip_serializing_if = "Option::is_none")]
    pub subclasses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thioesterases: Option<Vec<Thioesterase>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_at: Option<TransAT>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PksIterative {
    pub cyclization_type: String,
    #[serde(rename = "evidence", skip_serializing_if = "Option::is_none")]
    pub evidences: Option<Vec<IterativeEvidence>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nr_iterations: Option<u64>,
    pub subtype: IterativeSubtype,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum IterativeEvidence {
    #[serde(rename = "Sequence-based prediction")]
    SequenceBasedPrediction,
    #[serde(rename = "Structure-based inference")]
    StructureBasedInference,
    #[serde(rename = "Activity assay")]
    ActivityAssay,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum IterativeSubtype {
    #[serde(rename = "Partially reducing")]
    PartiallyReducing,
    #[serde(rename = "Non-reducing")]
    NonReducing,
    #[serde(rename = "Highly reducing")]
    HighlyReducing,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PksModule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_specificities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<SubstrateSpecificityEvidence>, // TODO: Should this be a list in the spec?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr_stereochem: Option<KrStereochemistry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_canonical: Option<NonCanonicalModule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pks_mod_doms: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum KrStereochemistry {
    Unknown,
    Inactive,
    #[serde(rename = "L-OH")]
    L,
    #[serde(rename = "D-OH")]
    D,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransAT {
    pub genes: Vec<String>,
}
