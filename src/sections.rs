use crate::attributes::attributes;
use crate::blocks::paragraph;
use crate::blocks::paragraphs;
use crate::blocks::Block;
use crate::containers::list_item;
use crate::containers::Container;
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};
use crate::attributes::AttributesObj;
use crate::blocks::list_paragraph;
// use std::collections::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::not_line_ending;
use nom::character::complete::space1;
use nom::bytes::complete::is_not;
use nom::sequence::tuple;
use nom::Parser;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Section {
    Aside {
        attributes: AttributesObj,
        content: Option<Vec<Block>>,
    },
    List {
        attributes: AttributesObj,
        items: Option<Vec<Container>>,
        preface: Option<Vec<Block>>,
    },
    P {
        attributes: AttributesObj,
        content: Option<Vec<Block>>,
    },
    // these get processed in the next step
    RawPageAttributes(
        Vec<(String, String)>
    ),

    Title {
        attributes: AttributesObj,
        content: Option<Vec<Block>>,
        headline: Option<Block>,
    },
}

pub fn sections(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, sections) = many1(preceded(multispace0, alt((
        aside, list, p, 
        raw_page_attributes, 
        title))))(source)?;
    Ok((source, sections))
}

pub fn aside(source: &str) -> IResult<&str, Section> {
    // Version 1 tests in place
    let (source, _) = tag_no_case("-- aside")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, attributes) = attributes(source)?;
    let (source, content) = paragraphs(source)?;
    Ok((
        source,
        Section::Aside {
            attributes,
            content,
        },
    ))
}


pub fn list(source: &str) -> IResult<&str, Section> {
    // Version 1 tests in place
    let (source, _) = tag_no_case("-- list")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, attributes) = attributes(source)?;
    let (source, preface) = opt(many1(preceded(multispace0, list_paragraph)))(source)?;
    let (source, items) = opt(many1(preceded(multispace0, list_item)))(source)?;
    Ok((
        source,
        Section::List {
            attributes,
            items,
            preface,
        },
    ))
}

pub fn p(source: &str) -> IResult<&str, Section> {
    // Version 1 tests in place
    let (source, _) = tag_no_case("-- p")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, attributes) = attributes(source)?;
    let (source, content) = paragraphs(source)?;
    Ok((
        source,
        Section::P {
            attributes,
            content,
        },
    ))
}

pub fn raw_page_attributes(source: &str) -> IResult<&str, Section> {
    let (source, _) = tag_no_case("-- attributes")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, page_attrs) = many1(
        tuple((
            tag("--"), 
            space1,
            is_not(":"),
            tag(":"),
            space1,
            not_line_ending, 
            opt(line_ending)
    )).map(|(_, _, key, _, _, value, _): (&str, &str, &str, &str, &str, &str, Option<&str>)| {
        (key.to_string(), value.to_string())
    }))(source)?;   
    Ok((
        source,
        Section::RawPageAttributes(page_attrs),
    ))
}

pub fn title(source: &str) -> IResult<&str, Section> {
    // Version 1 tests in place
    let (source, _) = tag_no_case("-- title")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, attributes) = attributes(source)?;
    let (source, headline) = opt(paragraph)(source)?;
    let (source, content) = paragraphs(source)?;
    Ok((
        source,
        Section::Title {
            attributes,
            headline,
            content,
        },
    ))
}

