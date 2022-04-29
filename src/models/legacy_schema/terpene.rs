use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Terpene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carbon_count_subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prenyltransferases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structural_subclass: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terpene_precursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terpene_synth_cycl: Option<Vec<String>>,
}
