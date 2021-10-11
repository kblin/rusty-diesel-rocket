use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Nrp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyclic: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lipid_moiety: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nrps_genes: Option<Vec<NrpsGene>>,
    #[serde(rename = "release_type", skip_serializing_if = "Option::is_none")]
    pub release_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subclass: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thioesterase: Option<Thioesterase>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NrpsGene {
    #[serde(rename = "gene_id")]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<NrpsModule>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NrpsModule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_substr_spec: Option<ADomainSubstrateSpecificity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_dom_subtype: Option<CDomainSubtype>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modification_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_canonical: Option<NonCanonicalModule>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ADomainSubstrateSpecificity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aa_subcluster: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epimerized: Option<bool>,
    #[serde(rename = "evidence", skip_serializing_if = "Option::is_none")]
    pub evidences: Option<Vec<SubstrateSpecificityEvidence>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonproteinogenic: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proteinogenic: Option<Vec<AminoAcid>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SubstrateSpecificityEvidence {
    #[serde(rename = "Sequence-based prediction")]
    SequenceBasedPrediction,
    #[serde(rename = "Structure-based inference")]
    StructureBasedInference,
    #[serde(rename = "Feeding study")]
    FeedingStudy,
    #[serde(rename = "Activity assay")]
    ActivityAssay,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AminoAcid {
    Alanine,
    Arginine,
    Glutamate,
    Serine,
    Tryptophan,
    Methionine,
    Threonine,
    Glycine,
    Isoleucine,
    Proline,
    Glutamine,
    Tyrosine,
    Phenylalanine,
    Cysteine,
    Histidine,
    Leucine,
    Lysine,
    Valine,
    Asparagine,
    Aspartate,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CDomainSubtype {
    Dual,
    Starter,
    LCL,
    Unknown,
    DCL,
    #[serde(rename = "Ester bond-forming")]
    EsterBondForming,
    Heterocyclization,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NonCanonicalModule {
    #[serde(rename = "evidence", skip_serializing_if = "Option::is_none")]
    pub evidences: Option<Vec<NonCanonicalEvidence>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_elongating: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum NonCanonicalEvidence {
    #[serde(rename = "Sequence-based prediction")]
    SequenceBasedPrediction,
    #[serde(rename = "Structure-based inference")]
    StructureBasedInference,
    #[serde(rename = "Activity assay")]
    ActivityAssay,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Thioesterase {
    pub gene: String,
    pub thioesterase_type: ThioesteraseType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ThioesteraseType {
    Unknown,
    #[serde(rename = "Type I")]
    TypeI,
    #[serde(rename = "Type II")]
    TypeII,
}
