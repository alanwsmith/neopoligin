use crate::attributes::AttributeV2;
use crate::blocks::Block;
use crate::helpers::empty_line::empty_line;
use crate::neo_sections::neo_section;
use crate::neo_sections::NeoSection;
use minijinja::value::{StructObject, Value};
use nom::character::complete::multispace0;
use nom::error::VerboseError;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub source_hash: Option<String>,
    pub path: Option<PathBuf>,
    pub source: Option<String>,
    pub section_storage: Option<Vec<NeoSection>>,
}

impl StructObject for Page {
    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "body_data" => Some(Value::from_serializable(&self.clone().body_data())),
            "title_data" => Some(Value::from_serializable(&self.clone().title_data())),
            "page_type" => Some(Value::from_serializable(&self.clone().page_type())),
            _ => None,
        }
    }
}

impl Page {
    pub fn body_data(&mut self) -> Vec<NeoSection> {
        self.raw_sections()
    }
}

impl Page {
    pub fn new_from(source: &str) -> Page {
        Page {
            path: None,
            source_hash: None,
            source: Some(source.to_string()),
            section_storage: None,
        }
    }
}

impl Page {
    pub fn page_type(&mut self) -> Option<String> {
        let metadata_section =
            self.raw_sections()
                .clone()
                .into_iter()
                .find_map(|s| match s.clone() {
                    NeoSection::MetaData { .. } => {
                        dbg!(&self.section_storage);
                        dbg!(&self.path);
                        dbg!(&s);
                        Some(s)
                    }
                    _ => None,
                });

        if let Some(metadata_items) = metadata_section {
            dbg!(metadata_items);

            // Some(s) => match s {
            //     NeoSection::MetaData { attributes } => Some("TTTTTTTTTTTTTT".to_string()),
            //     _ => Some("RRRRRRRRRRRRRRR".to_string()),
            // },
            // None => Some("EEEEEEEEEEEEEEEEEEEEEEEEE".to_string()),
        }

        Some("DDDDDDDDDDDDDDDDDDDDD".to_string())
    }
}

impl Page {
    pub fn raw_sections(&mut self) -> Vec<NeoSection> {
        match &self.section_storage {
            Some(x) => x.clone(),
            None => {
                self.section_storage = Some(page(self.source.clone().unwrap().as_str()).unwrap().1);
                self.section_storage.clone().unwrap()
            }
        }
    }
}

impl Page {
    pub fn template(&self) -> String {
        let attributes_section =
            self.clone()
                .raw_sections()
                .into_iter()
                .find_map(|s| match s.clone() {
                    NeoSection::RawPageAttributes { .. } => Some(s),
                    _ => None,
                });
        // dbg!(attributes_section);

        // match attributes_section {
        //     Some(s) => match s {
        //         NeoSection::RawPageAttributes{
        //         } => headline,
        //         _ => None,
        //     },
        //     None => None,
        // }

        "post".to_string()
    }
}

impl Page {
    pub fn title_data(&mut self) -> NeoSection {
        // TODO: Make this actually look for the title
        // instead of just the first thing
        self.raw_sections().into_iter().nth(0).unwrap()
    }
}

impl Page {
    pub fn title_string(&mut self) -> Option<Block> {
        let title_section = self
            .raw_sections()
            .into_iter()
            .find_map(|s| match s.clone() {
                NeoSection::Title { .. } => Some(s),
                _ => None,
            });

        match title_section {
            Some(s) => match s {
                NeoSection::Title {
                    attributes: _,
                    body: _,
                    headline,
                } => headline,
                _ => None,
            },
            None => None,
        }
    }
}

pub fn page(source: &str) -> IResult<&str, Vec<NeoSection>, VerboseError<&str>> {
    let (source, sections) =
        separated_list1(empty_line, preceded(multispace0, neo_section))(source)?;
    Ok((source, sections))
}
