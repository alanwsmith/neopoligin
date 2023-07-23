use crate::blocks::Block;
use crate::helpers::empty_line::empty_line;
use crate::neo_sections::neo_section;
use crate::neo_sections::NeoSection;
use nom::character::complete::multispace0;
use nom::error::VerboseError;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    // pub attributes: Option<HashMap<String, String>>,
    pub blurb: Option<Vec<Block>>,
    pub categories: Option<Vec<String>>,
    pub config: Option<HashMap<String, String>>,
    pub css: Option<Vec<String>>,
    pub date: Option<String>,
    pub head: Option<Vec<String>>,
    pub id: Option<String>,
    pub path: Option<PathBuf>,
    // pub references: Option<Vec<Reference>>,
    pub scripts: Option<Vec<String>>,
    pub sections: Option<Vec<NeoSection>>,
    pub source_hash: Option<String>,
    pub status: Option<String>,
    pub template: Option<String>,
    pub time: Option<String>,
    pub title: Option<Vec<Block>>,
    pub r#type: Option<String>,
}

impl Page {
    pub fn new_from(source: &str) -> Page {
        let mut p = Page {
            // attributes: None,
            blurb: None,
            categories: None,
            config: None,
            css: None,
            date: None,
            head: None,
            id: None,
            // references: None,
            path: None,
            sections: None,
            scripts: None,
            source_hash: None,
            status: None,
            template: None,
            time: None,
            title: None,
            r#type: None,
        };
        let raw_sections = page(source).unwrap().1;
        let filtered_sections: Option<Vec<NeoSection>> = Some(
            raw_sections
                .into_iter()
                .filter_map(|sec| match sec {
                    NeoSection::RawPageAttributes(key_values) => {
                        key_values.iter().for_each(|(key, value)| {
                            match key.to_lowercase().trim() {
                                "date" => {
                                    p.date = Some(value.trim().to_string());
                                }
                                "id" => {
                                    p.id = Some(value.trim().to_string());
                                }
                                "status" => {
                                    p.status = Some(value.trim().to_string());
                                }
                                "time" => {
                                    p.time = Some(value.trim().to_string());
                                }
                                "type" => {
                                    p.r#type = Some(value.trim().to_string());
                                }
                                _ => {}
                            }
                            ()
                        });
                        None
                    }
                    x => Some(x),
                })
                .collect(),
        );
        if filtered_sections.as_ref().unwrap().len() == 0 {
            p.sections = None;
        } else {
            p.sections = filtered_sections;
        }
        p
    }
}

pub fn page(source: &str) -> IResult<&str, Vec<NeoSection>, VerboseError<&str>> {
    let (source, sections) =
        separated_list1(empty_line, preceded(multispace0, neo_section))(source)?;
    Ok((source, sections))
}
