use crate::attributes::attributes;
use crate::attributes::attributes2;
use crate::attributes::Attribute;
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Section {
    Aside {
        attributes: AttributesObj,
        content: Option<Vec<Block>>,
    },
    List {
        attributes: Option<Vec<Attribute>>,
        items: Option<Vec<Container>>,
        preface: Option<Vec<Block>>,
    },
    P {
        attributes: AttributesObj,
        content: Option<Vec<Block>>,
    },
    Title {
        attributes: Option<Vec<Attribute>>,
        content: Option<Vec<Block>>,
        headline: Option<Block>,
    },
}

pub fn sections(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, sections) = many1(preceded(multispace0, alt((aside, list, p, title))))(source)?;
    Ok((source, sections))
}

pub fn aside(source: &str) -> IResult<&str, Section> {
    let (source, _) = tag_no_case("-- aside")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, attributes) = attributes2(source)?;
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
    let (source, _) = tag_no_case("-- list")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, attributes) = attributes(source)?;
    let preface = None; // TODO
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
    let (source, _) = tag_no_case("-- p")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, attributes) = attributes2(source)?;
    let (source, content) = paragraphs(source)?;
    Ok((
        source,
        Section::P {
            attributes,
            content,
        },
    ))
}




pub fn title(source: &str) -> IResult<&str, Section> {
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
