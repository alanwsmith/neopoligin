#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::{Deserialize, Serialize};
use crate::attributes::attributes;
use crate::attributes::Attribute;




pub fn aside(source: &str) -> IResult<&str, Section> {
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
    let (source, _) = tag_no_case("-- list")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = line_ending(source)?;
    let (source, attributes) = attributes(source)?;
    let preface = None;
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

pub fn list_item(source: &str) -> IResult<&str, Container> {
    let (source, content) = many1(preceded(tag("- "), paragraph))(source)?;
    Ok((source, Container::ListItem { content }))
}

pub fn sections(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, sections) = many1(preceded(multispace0, alt((aside, list, p, title))))(source)?;
    Ok((source, sections))
}

pub fn snippet_strong(source: &str) -> IResult<&str, Snippet> {
    let (source, _) = tag_no_case("<<strong|")(source)?;
    let (source, content) = is_not("|>")(source)?;
    dbg!(&source);
    let (source, attributes) = attributes(source)?;
    dbg!(&source);
    let (source, _) = tag(">>")(source)?;
    Ok((
        source,
        Snippet::Strong {
            string: content.to_string(),
            attributes,
        },
    ))
}

pub fn snippet_text(source: &str) -> IResult<&str, Snippet> {
    let (source, content) = is_not("\n<")(source)?;
    Ok((
        source,
        Snippet::Text {
            string: content.to_string(),
        },
    ))
}

pub fn p(source: &str) -> IResult<&str, Section> {
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

pub fn paragraphs(source: &str) -> IResult<&str, Option<Vec<Block>>> {
    let (source, paragraphs) = opt(many1(preceded(multispace0, paragraph)))(source)?;
    Ok((source, paragraphs))
}

pub fn paragraph(source: &str) -> IResult<&str, Block> {
    let (source, _) = not(tag("--"))(source)?;
    let (source, snippets) = many1(preceded(
        opt(line_ending),
        alt((snippet_text, snippet_strong)),
    ))(source)?;
    Ok((source, Block::Paragraph { snippets }))
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



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Section {
    Aside {
        attributes: Option<Vec<Attribute>>,
        content: Option<Vec<Block>>,
    },
    List {
        attributes: Option<Vec<Attribute>>,
        preface: Option<Vec<Block>>,
        items: Option<Vec<Container>>,
    },
    P {
        attributes: Option<Vec<Attribute>>,
        content: Option<Vec<Block>>,
    },
    Title {
        attributes: Option<Vec<Attribute>>,
        headline: Option<Block>,
        content: Option<Vec<Block>>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    Paragraph { snippets: Vec<Snippet> },
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Container {
    ListItem { content: Vec<Block> },
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Snippet {
    Text {
        string: String,
    },
    Strong {
        string: String,
        attributes: Option<Vec<Attribute>>,
    },
    None,
}
