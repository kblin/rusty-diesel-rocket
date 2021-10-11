use super::publication::Publication;
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Genes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotations>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_genes: Option<Vec<ExtraGene>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operons: Option<Vec<Operon>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Annotations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<GeneFunctions>>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mut_pheno: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<Publication>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tailoring: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneFunctions {
    pub category: String,
    #[serde(rename = "evidence")]
    pub evidences: Vec<GeneFunctionEvidence>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GeneFunctionEvidence {
    #[serde(rename = "Sequence-based prediction")]
    SequenceBasedPrediction,
    #[serde(rename = "Other in vivo study")]
    OtherInVivoStudy,
    #[serde(rename = "Heterologous expression")]
    HeterologousExpression,
    #[serde(rename = "Knock-out")]
    KnockOut,
    #[serde(rename = "Activity assay")]
    ActivityAssay,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtraGene {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub exons: Vec<Exon>,
    pub strand: Strand,
}

#[derive(Debug)]
pub enum Strand {
    Forward,
    Reverse,
}

impl Serialize for Strand {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let t = match self {
            Strand::Forward => 1,
            Strand::Reverse => -1,
        };
        serializer.serialize_i8(t)
    }
}

impl<'de> Deserialize<'de> for Strand {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StrandVisitor;
        impl<'de> Visitor<'de> for StrandVisitor {
            type Value = Strand;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("either 1 or -1")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Strand, E>
            where
                E: de::Error,
            {
                let s = match value {
                    1 => Strand::Forward,
                    -1 => Strand::Reverse,
                    _ => return Err(E::custom("invalid strand value")),
                };
                Ok(s)
            }

            // The JSON deserializer insists on calling visit_u64 for unsigend ints like 1
            fn visit_u64<E>(self, value: u64) -> Result<Strand, E>
            where
                E: de::Error,
            {
                let s = match value {
                    1 => Strand::Forward,
                    _ => return Err(E::custom("invalid strand value")),
                };
                Ok(s)
            }
        }

        deserializer.deserialize_i64(StrandVisitor)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Exon {
    pub start: i64,
    pub end: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Operon {
    #[serde(rename = "evidence")]
    pub evidences: Vec<OperonEvidence>,
    pub genes: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OperonEvidence {
    #[serde(rename = "Sequence-based prediction")]
    SequenceBasedPrediction,
    RACE,
    ChIPseq,
    RNAseq,
}
