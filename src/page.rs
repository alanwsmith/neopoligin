#![allow(unused_imports)]
use crate::attributes::*;
use crate::blocks::*;
use crate::helpers::empty_line::empty_line;
use crate::helpers::get_image_path::get_image_path;
use crate::neo_sections::metadata_section::MetadataItem;
use crate::neo_sections::neo_section;
use crate::neo_sections::NeoSection;
use minijinja::value::{StructObject, Value};
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
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
            "source_data" => Some(Value::from_serializable(&self.clone().source_data())),
            "title_data" => Some(Value::from_serializable(&self.clone().title_data())),
            "page_type" => Some(Value::from_serializable(&self.clone().page_type())),
            "css_blocks" => Some(Value::from_serializable(&self.clone().css_blocks())),
            "script_blocks" => Some(Value::from_serializable(&self.clone().script_blocks())),
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
    pub fn source_data(&mut self) -> Option<String> {
        self.clone().source
    }
}

impl Page {
    pub fn new_from(source: &str) -> Page {
        // dbg!(&source);
        Page {
            path: None,
            source_hash: None,
            source: Some(flatten(source).unwrap().1),
            section_storage: None,
        }
    }
}

// remove all the double newlines making sure to
// move content from single newlines into place
// properly
fn flatten(source: &str) -> IResult<&str, String> {
    // dbg!(&source);
    let (source, value) = many1(alt((
        startcode_section,
        startcss_section,
        startresults_section,
        attr_line,
        multi_line,
        line,
    )))(source)?;
    let mut response = value.join("\n");
    response.push_str("\n");
    response.push_str(source);
    Ok(("", response))
}

fn startcode_section(source: &str) -> IResult<&str, String> {
    let (source, _) = tag("-- startcode")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- endcode")(source)?;
    Ok((source, format!("-- startcode\n{}", body)))
}

fn startcss_section(source: &str) -> IResult<&str, String> {
    let (source, _) = tag("-- startcss")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- endcss")(source)?;
    Ok((source, format!("-- startcss\n{}", body.trim())))
}

fn startresults_section(source: &str) -> IResult<&str, String> {
    let (source, _) = tag("-- startresults")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- endresults")(source)?;
    Ok((source, format!("-- startresults\n{}", body)))
}

fn attr_line(source: &str) -> IResult<&str, String> {
    let (source, captured) = pair(tag("-- "), is_not("\n"))(source)?;
    let (source, _) = tag("\n")(source)?;
    let (source, _) = multispace0(source)?;
    Ok((source, format!("{}{}", captured.0, captured.1)))
}

fn line(source: &str) -> IResult<&str, String> {
    // dbg!(&source);
    let (source, value) = is_not("\n")(source)?;
    let (source, _) = tag("\n")(source)?;
    // dbg!(&source);
    let (source, _) = pair(space0, line_ending)(source)?;
    // dbg!(&source);
    let (source, _) = multispace0(source)?;
    // dbg!(&source);
    Ok((source, value.to_string()))
}

fn multi_line(source: &str) -> IResult<&str, String> {
    // dbg!(source);
    // dbg!(&source);
    let (source, value) = many1(pair(is_not("\n"), tag("\n")).map(|(a, _b)| a))(source)?;
    // dbg!(&source);
    let (source, _) = multispace0(source)?;
    // dbg!(&source);
    Ok((source, value.join(" ")))
}

impl Page {
    pub fn page_type(&mut self) -> Option<String> {
        //     if let Some(metadata_section) =
        //         self.raw_sections()
        //             .clone()
        //             .into_iter()
        //             .find_map(|s| match s.clone() {
        //                 NeoSection::MetaData { .. } => Some(s),
        //                 _ => None,
        //             })
        //     {
        //         match metadata_section {
        //             NeoSection::MetaData { attributes } => {
        //                 attributes
        //                     .unwrap()
        //                     .into_iter()
        //                     .find_map(|a| match a.clone() {
        //                         AttributeV2::Type(x) => {
        //                             dbg!(&x);
        //                             Some(x.trim().to_string())
        //                         }
        //                         _ => None,
        //                     })
        //             }
        //             _ => None,
        //         }
        //     } else {
        //         None
        //     }

        None
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

// impl Page {
//     pub fn template(&self) -> String {
//         // let attributes_section =
//         //     self.clone()
//         //         .raw_sections()
//         //         .into_iter()
//         //         .find_map(|s| match s.clone() {
//         //             NeoSection::RawPageAttributes { .. } => Some(s),
//         //             _ => None,
//         //         });
//         // dbg!(attributes_section);
//         // match attributes_section {
//         //     Some(s) => match s {
//         //         NeoSection::RawPageAttributes{
//         //         } => headline,
//         //         _ => None,
//         //     },
//         //     None => None,
//         // }
//         "post".to_string()
//     }
// }

impl Page {
    pub fn title_data(&mut self) -> NeoSection {
        // TODO: Make this actually look for the title
        // instead of just the first thing
        self.raw_sections().into_iter().nth(0).unwrap()
    }
}

impl Page {
    pub fn load_image_paths(&mut self) {
        self.raw_sections();
        self.section_storage.iter_mut().for_each(|top| {
            top.iter_mut().for_each(|s| match s {
                NeoSection::Image {
                    ref mut src, name, ..
                } => {
                    if let Some(n) = name {
                        // let newthing = n.to_string();
                        *src = get_image_path(n);
                        // dbg!(&src);
                    }
                    ()
                }
                _ => (),
            })
        });
    }
}

impl Page {
    pub fn css_blocks(&mut self) -> Vec<String> {
        let mut response = vec![];
        self.raw_sections()
            .into_iter()
            .find_map(|s| match s.clone() {
                NeoSection::Css { body, .. } => Some(response.push(body.unwrap())),
                _ => None,
            });
        response
    }
}

impl Page {
    pub fn script_blocks(&mut self) -> Vec<String> {
        let mut response = vec![];
        self.raw_sections()
            .into_iter()
            .find_map(|s| match s.clone() {
                NeoSection::Script { body, .. } => Some(response.push(body.unwrap())),
                _ => None,
            });
        response
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
    // dbg!(&source);
    let (source, sections) = many1(neo_section)(source)?;
    //separated_list1(empty_line, preceded(multispace0, neo_section))(source)?;
    // dbg!(&source);
    Ok((source, sections))
}
