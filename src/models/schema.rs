use serde::{Deserialize, Serialize};

use super::super::utils;

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    changelog: Vec<ChangeLog>,
    cluster: Cluster,
    #[serde(skip_serializing_if = "Option::is_none")]
    comments: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cluster {
    biosyn_class: Vec<BiosyntheticClass>,
    loci: Loci,
    mibig_accession: String,
    minimal: bool,
    #[serde(with = "utils::num_as_string")]
    ncbi_tax_id: u64,
    organism_name: String,
    publications: Vec<Publication>,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug)]
pub enum PublicationType {
    Pubmed,
    Doi,
    Patent,
    Url,
}

#[derive(Debug)]
pub struct Publication {
    id_type: PublicationType,
    id: String,
}

use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use std::fmt;

impl Serialize for Publication {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let t = match self.id_type {
            PublicationType::Pubmed => "pubmed",
            PublicationType::Doi => "doi",
            PublicationType::Url => "url",
            PublicationType::Patent => "patent",
        };
        let s = format!("{}:{}", t, self.id);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Publication {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PublicationVisitor;
        impl<'de> Visitor<'de> for PublicationVisitor {
            type Value = Publication;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Publication")
            }

            fn visit_str<E>(self, value: &str) -> Result<Publication, E>
            where
                E: de::Error,
            {
                let v: Vec<&str> = value.splitn(2, ':').collect();
                if v.len() == 2 {
                    let t = match v[0] {
                        "pubmed" => PublicationType::Pubmed,
                        "doi" => PublicationType::Doi,
                        "url" => PublicationType::Url,
                        "patent" => PublicationType::Patent,
                        _ => return Err(E::custom("invalid publication type")),
                    };

                    Ok(Publication {
                        id_type: t,
                        id: v[1].to_string(),
                    })
                } else {
                    Err(E::custom("invalid publication format"))
                }
            }
        }

        deserializer.deserialize_str(PublicationVisitor)
    }
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
