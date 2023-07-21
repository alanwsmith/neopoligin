use serde::{Deserialize, Serialize};
use crate::blocks::Block;
use std::collections::HashMap;
use crate::sections::Section;
use nom::IResult;
use crate::sections::sections;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct Page {
    attributes: Option<HashMap<String, String>>,
    blurb: Option<Vec<Block>>,
    categories: Option<Vec<String>>,
    config: Option<HashMap<String, String>>,
    css: Option<Vec<String>>,
    head: Option<Vec<String>>,
    references: Option<Vec<Reference>>,
    scripts: Option<Vec<String>>,
    title: Option<Vec<Block>>,
    sections: Vec<Section>
}

impl Page {
    pub fn new() -> Page {
        Page{
            attributes: None,
            blurb: None,
            categories: None,
            config: None,
            css: None,
            head: None,
            references: None,
            scripts: None,
            title: None,
            sections: vec![]
        }
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
