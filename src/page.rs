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
    pub source: String,
    pub section_storage: Option<Vec<NeoSection>>,
}

impl StructObject for Page {
    fn get_field(&self, field: &str) -> Option<Value> {
        match field {
            "body_data" => Some(Value::from_serializable(&self.clone().body_data())),
            "title_data" => Some(Value::from_serializable(&self.clone().title_data())),
            _ => None,
        }
    }
}

impl Page {
    pub fn new_from(source: &str) -> Page {
        Page {
            path: None,
            source_hash: None,
            source: source.to_string(),
            section_storage: None,
        }
    }
}

impl Page {
    pub fn raw_sections(&mut self) -> Vec<NeoSection> {
        match &self.section_storage {
            Some(x) => x.clone(),
            None => {
                self.section_storage = Some(page(self.source.as_str()).unwrap().1);
                self.section_storage.clone().unwrap()
            }
        }
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
                    attributes,
                    content,
                    headline,
                } => headline,
                _ => None,
            },

            None => None,
        }

        // if let Some(s) = title_section {
        //     dbg!(s.s);
        //     "asdf".to_string()
        // } else {
        // }
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
    pub fn body_data(&mut self) -> Vec<NeoSection> {
        self.raw_sections()
    }
}

pub fn page(source: &str) -> IResult<&str, Vec<NeoSection>, VerboseError<&str>> {
    let (source, sections) =
        separated_list1(empty_line, preceded(multispace0, neo_section))(source)?;
    Ok((source, sections))
}
