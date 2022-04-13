use super::publication::Publication;
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Compound {
    #[serde(rename = "chem_acts", skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<String>>,
    #[serde(rename = "chem_moieties", skip_serializing_if = "Option::is_none")]
    pub moieties: Option<Vec<Moiety>>,
    #[serde(rename = "chem_struct", skip_serializing_if = "Option::is_none")]
    pub structure: Option<String>,
    #[serde(rename = "chem_synonyms", skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    #[serde(rename = "chem_targets", skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
    #[serde(rename = "compound")]
    pub name: String,
    #[serde(rename = "database_id", skip_serializing_if = "Option::is_none")]
    pub database_ids: Option<Vec<DatabaseId>>,
    #[serde(rename = "evidence", skip_serializing_if = "Option::is_none")]
    pub evidences: Option<Vec<CompoundEvidence>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mass_spec_ion_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mol_mass: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_formula: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Moiety {
    #[serde(rename = "moiety")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcluster: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Target {
    #[serde(rename = "target")]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<Publication>>,
}

#[derive(Debug)]
pub enum DatabaseType {
    NPAtlas,
    PubChem,
    ChEBI,
    ChEMBL,
    ChemSpider,
}

#[derive(Debug)]
pub struct DatabaseId {
    pub id_type: DatabaseType,
    pub id: String,
}

impl Serialize for DatabaseId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let t = match self.id_type {
            DatabaseType::ChEBI => "chebi",
            DatabaseType::ChEMBL => "chembl",
            DatabaseType::ChemSpider => "chemspider",
            DatabaseType::NPAtlas => "npatlas",
            DatabaseType::PubChem => "pubchem",
        };
        let s = format!("{}:{}", t, self.id);
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for DatabaseId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DatabaseIdVisitor;
        impl<'de> Visitor<'de> for DatabaseIdVisitor {
            type Value = DatabaseId;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct DatabaseId")
            }

            fn visit_str<E>(self, value: &str) -> Result<DatabaseId, E>
            where
                E: de::Error,
            {
                let v: Vec<&str> = value.splitn(2, ':').collect();
                if v.len() == 2 {
                    let t = match v[0] {
                        "chebi" => DatabaseType::ChEBI,
                        "chembl" => DatabaseType::ChEMBL,
                        "chemspider" => DatabaseType::ChemSpider,
                        "npatlas" => DatabaseType::NPAtlas,
                        "pubchem" => DatabaseType::PubChem,
                        _ => return Err(E::custom("invalid database id type")),
                    };

                    Ok(DatabaseId {
                        id_type: t,
                        id: v[1].to_string(),
                    })
                } else {
                    Err(E::custom("invalid database id format"))
                }
            }
        }

        deserializer.deserialize_str(DatabaseIdVisitor)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CompoundEvidence {
    #[serde(rename = "Chemical derivatization")]
    ChemicalDerivitization,
    #[serde(rename = "Total synthesis")]
    TotalSynthesis,
    #[serde(rename = "Mass spectrometry")]
    MassSpectrometry,
    #[serde(rename = "X-ray")]
    XRay,
    NMR,
}
