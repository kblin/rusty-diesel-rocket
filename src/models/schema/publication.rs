use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub enum PublicationType {
    Pubmed,
    Doi,
    Patent,
    Url,
}

#[derive(Debug)]
pub struct Publication {
    pub id_type: PublicationType,
    pub id: String,
}

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
