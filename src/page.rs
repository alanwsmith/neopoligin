use crate::blocks::Block;
use crate::sections::sections;
use crate::sections::Section;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    pub attributes: Option<HashMap<String, String>>,
    pub blurb: Option<Vec<Block>>,
    pub categories: Option<Vec<String>>,
    pub config: Option<HashMap<String, String>>,
    pub css: Option<Vec<String>>,
    pub head: Option<Vec<String>>,
    pub id: Option<String>,
    pub path: Option<PathBuf>,
    pub references: Option<Vec<Reference>>,
    pub scripts: Option<Vec<String>>,
    pub sections: Option<Vec<Section>>,
    pub template: Option<String>,
    pub title: Option<Vec<Block>>,
    pub r#type: Option<String>,
}

impl Page {
    pub fn new() -> Page {
        Page {
            attributes: None,
            blurb: None,
            categories: None,
            config: None,
            css: None,
            head: None,
            id: None,
            references: None,
            path: None,
            sections: None,
            scripts: None,
            template: None,
            title: None,
            r#type: None,
        }
    }
}

impl Page {
    pub fn new_from(source: &str) -> Page {
        let mut p = Page::new();
        let raw_sections = sections(source).unwrap().1;
        let filtered_sections: Option<Vec<Section>> = Some(
            raw_sections
                .into_iter()
                .filter_map(|sec| match sec {
                    Section::RawPageAttributes(key_values) => {
                        key_values.iter().for_each(|(key, value)| {
                            match key.to_lowercase().trim() {
                                "id" => {
                                    p.id = Some(value.trim().to_string());
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reference {}

pub fn page(source: &str) -> IResult<&str, Page> {
    let mut p = Page::new();
    let raw_sections = sections(source).unwrap().1;
    p.sections = Some(raw_sections);
    Ok((source, p))
}
