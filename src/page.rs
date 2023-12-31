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
use nom::combinator::not;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::multi::many0;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
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
            "highlight_html" => Some(Value::from_serializable(&self.clone().highlight_html())),
            _ => None,
        }
    }
}


impl Page {
    pub fn highlight_html(&mut self) -> String {
        "HLLLLLOOOOOOOOIOOOO".to_string()
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
    let (source, value) = many1(alt((
        code_section,
        codestart_section,
        css_section,
        cssstart_section,
        html_section,
        htmlstart_section,
        pre_section,
        prestart_section,
        resultsstart_section,
        script_section,
        scriptstart_section,
        //
        attr_line,
        line,
        multi_line,
    )))(source)?;
    let mut response = value.join("\n");
    response.push_str("\n");
    response.push_str(source);
    Ok(("", response))
}

fn code_section(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-- code")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attrs) = many0(tuple((tag("-- "), not_line_ending, line_ending))
        .map(|x| format!("{}{}{}", x.0, x.1, x.2)))(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- ")(source)?;
    let response = format!("-- code\n{}{}", attrs.join(""), body);
    Ok((source, response))
}

fn codestart_section(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-- code/")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- /code")(source)?;
    Ok((source, format!("-- code/\n{}", body)))
}

fn css_section(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-- css")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attrs) = many0(tuple((tag("-- "), not_line_ending, line_ending))
        .map(|x| format!("{}{}{}", x.0, x.1, x.2)))(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- ")(source)?;
    let response = format!("-- css\n{}{}", attrs.join(""), body);
    Ok((source, response))
}

fn cssstart_section(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-- css/")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- /css")(source)?;
    Ok((source, format!("-- css/\n{}", body.trim())))
}

fn html_section(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-- html")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attrs) = many0(tuple((tag("-- "), not_line_ending, line_ending))
        .map(|x| format!("{}{}{}", x.0, x.1, x.2)))(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- ")(source)?;
    let response = format!("-- html\n{}{}", attrs.join(""), body);
    Ok((source, response))
}

fn htmlstart_section(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-- html/")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- /html")(source)?;
    Ok((source, format!("-- html/\n{}", body.trim())))
}



fn pre_section(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-- pre")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attrs) = many0(tuple((tag("-- "), not_line_ending, line_ending))
        .map(|x| format!("{}{}{}", x.0, x.1, x.2)))(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- ")(source)?;
    let response = format!("-- pre\n{}{}", attrs.join(""), body);
    Ok((source, response))
}

fn prestart_section(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-- pre/")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- /pre")(source)?;
    Ok((source, format!("-- pre/\n{}", body.trim())))
}

fn resultsstart_section(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-- results/")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- /results")(source)?;
    Ok((source, format!("-- results/\n{}", body)))
}

fn script_section(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-- script")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attrs) = many0(tuple((tag("-- "), not_line_ending, line_ending))
        .map(|x| format!("{}{}{}", x.0, x.1, x.2)))(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- ")(source)?;
    let response = format!("-- script\n{}{}", attrs.join(""), body);
    Ok((source, response))
}

fn scriptstart_section(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("-- script/")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = take_until("-- /script")(source)?;
    Ok((source, format!("-- script/\n{}", body.trim())))
}


fn attr_line(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, captured) = pair(tag("-- "), is_not("\n"))(source)?;
    let (source, _) = tag("\n")(source)?;
    let (source, _) = multispace0(source)?;
    Ok((source, format!("{}{}", captured.0, captured.1)))
}

fn line(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = not(tag("--"))(source)?;
    let (source, value) = is_not("\n")(source)?;
    let (source, _) = tag("\n")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, _) = multispace0(source)?;
    Ok((source, value.to_string()))
}

fn multi_line(source: &str) -> IResult<&str, String> {
    let (source, _) = multispace0(source)?;
    let (source, _) = not(tag("--"))(source)?;
    let (source, value) = many1(pair(is_not("\n"), tag("\n")).map(|(a, _b)| a))(source)?;
    let (source, _) = multispace0(source)?;
    Ok((source, value.join(" ")))
}

impl Page {
    pub fn page_type(&mut self) -> Option<String> {
        if let Some(metadata_section) =
            self.raw_sections()
                .clone()
                .into_iter()
                .find_map(|s| match s.clone() {
                    NeoSection::Metadata { .. } => Some(s),
                    _ => None,
                })
        {
            match metadata_section {
                NeoSection::Metadata { list } => {
                    list.unwrap().into_iter().find_map(|a| match a.clone() {
                        MetadataItem::Type(x) => {
                            Some(x.trim().to_string())
                        }
                        _ => None,
                    })
                }
                _ => None,
            }
        } else {
            None
        }
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
        self.raw_sections()
            .into_iter()
            .filter_map(|s| match s.clone() {
                NeoSection::Css { body, .. } => Some(body.unwrap()),
                _ => None,
            })
            .collect()
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
    let (source, sections) = many1(neo_section)(source)?;
    //separated_list1(empty_line, preceded(multispace0, neo_section))(source)?;
    Ok((source, sections))
}
