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
    pub path: Option<PathBuf>,
    pub references: Option<Vec<Reference>>,
    pub scripts: Option<Vec<String>>,
    pub sections: Vec<Section>,
    pub title: Option<Vec<Block>>,
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
            references: None,
            path: None,
            sections: vec![],
            scripts: None,
            title: None,
        }
    }
}

impl Page {
    pub fn new_from(source: &str) -> Page {
        let mut p = Page::new();
        let raw_sections = sections(source).unwrap().1;
        p.sections = raw_sections;
        p
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Reference {}

pub fn page(source: &str) -> IResult<&str, Page> {
    let mut p = Page::new();
    let raw_sections = sections(source).unwrap().1;
    p.sections = raw_sections;
    Ok((source, p))
}
