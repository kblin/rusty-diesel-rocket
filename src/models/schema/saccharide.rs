use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Saccharide {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glycosyltransferases: Option<Vec<Glycosyltransferase>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subclass: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sugar_subclusters: Option<Vec<Vec<String>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Glycosyltransferase {
    #[serde(rename = "evidence")]
    evidences: Vec<GTEvidence>,
    gene_id: String,
    specificity: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GTEvidence {
    #[serde(rename = "Sequence-based prediction")]
    SequenceBasedPrediction,
    #[serde(rename = "Structure-based inference")]
    StructureBasedInference,
    #[serde(rename = "Knock-out construct")]
    KnockOutConstruct,
    #[serde(rename = "Activity assay")]
    ActivityAssay,
}
