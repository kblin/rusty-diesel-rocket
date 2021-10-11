use serde::{Deserialize, Serialize};

use super::super::utils;

pub mod alkaloid;
pub mod compound;
pub mod genes;
pub mod nrp;
pub mod publication;
pub mod ripp;

pub use alkaloid::Alkaloid;
pub use compound::Compound;
pub use genes::Genes;
pub use nrp::Nrp;
pub use publication::{Publication, PublicationType};
pub use ripp::RiPP;

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub changelog: Vec<ChangeLog>,
    pub cluster: Cluster,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cluster {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alkaloid: Option<Alkaloid>,
    pub biosyn_class: Vec<BiosyntheticClass>,
    pub compounds: Vec<Compound>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genes: Option<Genes>,
    pub loci: Loci,
    pub mibig_accession: String,
    pub minimal: bool,
    #[serde(with = "utils::num_as_string")]
    pub ncbi_tax_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nrp: Option<Nrp>,
    pub organism_name: String,
    pub publications: Vec<Publication>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ripp: Option<RiPP>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum BiosyntheticClass {
    Alkaloid,
    NRP,
    Polyketide,
    RiPP,
    Saccharide,
    Terpene,
    Other,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChangeLog {
    comments: Vec<String>,
    contributors: Vec<String>,
    version: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LociCompleteness {
    Incomplete,
    Complete,
    #[serde(rename = "Unknown")]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum LociEvidence {
    #[serde(rename = "Sequence-based prediction")]
    SequenceBasedPrediction,
    #[serde(rename = "Gene expression correlated with compound production")]
    GeneExpressionCorrelatedWithCompoundProduction,
    #[serde(rename = "Knock-out studies")]
    KnockOutStudies,
    #[serde(rename = "Enzymatic assays")]
    EnzymaticAssays,
    #[serde(rename = "Heterologous expression")]
    HeterologousExpression,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Loci {
    accession: String,
    completeness: LociCompleteness,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_coord: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    evidence: Option<Vec<LociEvidence>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mixs_compliant: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start_coord: Option<u64>,
}
