#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
use crate::attributes::attributes;
use crate::attributes::AttributesObj;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::error::VerboseErrorKind;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

fn line_cleanup(source: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let (source, _) = pair(space0, line_ending)(source)?;
    Ok((source, ""))
}

fn empty_line(source: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let (source, _) = pair(space0, line_ending)(source)?;
    Ok((source, ""))
}

fn spacer_line(source: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let (source, _) = pair(line_ending, line_ending)(source)?;
    Ok((source, ""))
}

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
    pub sections: Option<Vec<Section>>,
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
        let raw_sections = parse(source).unwrap().1;
        let filtered_sections: Option<Vec<Section>> = Some(
            raw_sections
                .into_iter()
                .filter_map(|sec| match sec {
                    Section::RawPageAttributes(key_values) => {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Section {
    Aside {
        attributes: Option<AttributesObj>,
        content: Option<Vec<Block>>,
    },
    Title {
        attributes: Option<AttributesObj>,
        content: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    P {
        attributes: Option<AttributesObj>,
        content: Option<Vec<Block>>,
    },
    RawPageAttributes(Vec<(String, String)>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    Paragraph { content: Vec<Token> },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Token {
    Link { url: String, content: String },
    Space,
    Text { content: String },
}

fn parse(source: &str) -> IResult<&str, Vec<Section>, VerboseError<&str>> {
    let (source, sections) = separated_list1(empty_line, preceded(multispace0, section))(source)?;
    Ok((source, sections))
}

fn section(source: &str) -> IResult<&str, Section, VerboseError<&str>> {
    let (source, section) = alt((aside_section, p_section, title_section))(source)?;
    Ok((source, section))
}

fn aside_section(source: &str) -> IResult<&str, Section, VerboseError<&str>> {
    let (source, _) = tag("-- aside")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = attributes(source)?;
    let (source, _) = empty_line(source)?;
    let (source, content) = opt(many1(block))(source)?;
    Ok((
        source,
        Section::Aside {
            attributes: Some(attributes),
            content,
        },
    ))
}

fn p_section(source: &str) -> IResult<&str, Section, VerboseError<&str>> {
    let (source, _) = tag("-- p")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = attributes(source)?;
    let (source, _) = empty_line(source)?;
    let (source, content) = opt(many1(block))(source)?;
    Ok((
        source,
        Section::P {
            attributes: Some(attributes),
            content,
        },
    ))
}

fn title_section(source: &str) -> IResult<&str, Section, VerboseError<&str>> {
    let (source, _) = tag("-- title")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = attributes(source)?;
    let (source, _) = empty_line(source)?;
    let (source, headline) = opt(block)(source)?;
    let (source, content) = opt(many1(block))(source)?;
    Ok((
        source,
        Section::Title {
            attributes: Some(attributes),
            content,
            headline,
        },
    ))
}

fn block(source: &str) -> IResult<&str, Block, VerboseError<&str>> {
    let (source, _) = not(pair(multispace0, tag("--")))(source)?;
    let (source, block) = preceded(multispace0, paragraph_block)(source)?;
    Ok((source, block))
}

fn paragraph_block(source: &str) -> IResult<&str, Block, VerboseError<&str>> {
    let (source, content) = many1(token)(source)?;
    Ok((source, Block::Paragraph { content }))
}

fn token(source: &str) -> IResult<&str, Token, VerboseError<&str>> {
    let (source, _) = not(spacer_line)(source)?;
    let (source, token) = alt((single_newline, text_token, link_token, space_token))(source)?;
    Ok((source, token))
}

fn single_newline(source: &str) -> IResult<&str, Token, VerboseError<&str>> {
    let (source, _) = pair(line_ending, not(line_ending))(source)?;
    Ok((source, Token::Space))
}


fn space_token(source: &str) -> IResult<&str, Token, VerboseError<&str>> {
    let (source, _) = space1(source)?;
    Ok((source, Token::Space))
}

fn text_token(source: &str) -> IResult<&str, Token, VerboseError<&str>> {
    let (source, token) = is_not("< \n\t\r")(source)?;
    Ok((
        source,
        Token::Text {
            content: token.to_string(),
        },
    ))
}

fn link_token(source: &str) -> IResult<&str, Token, VerboseError<&str>> {
    let (source, _) = tag("<<link|")(source)?;
    let (source, text) = is_not("|")(source)?;
    let (source, _) = tag("|")(source)?;
    let (source, url) = is_not(">")(source)?;
    let (source, _) = tag(">>")(source)?;
    Ok((
        source,
        Token::Link {
            content: text.to_string(),
            url: url.to_string(),
        },
    ))
}
