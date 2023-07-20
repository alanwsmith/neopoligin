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
use crate::snippets::Snippet;
use crate::containers::Container;
use crate::snippets::strong;

use crate::snippets::text;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Block {
    Paragraph { snippets: Vec<Snippet> },
    None,
}


pub fn paragraphs(source: &str) -> IResult<&str, Option<Vec<Block>>> {
    let (source, paragraphs) = opt(many1(preceded(multispace0, paragraph)))(source)?;
    Ok((source, paragraphs))
}

pub fn paragraph(source: &str) -> IResult<&str, Block> {
    let (source, _) = not(tag("--"))(source)?;
    let (source, snippets) = many1(preceded(
        opt(line_ending),
        alt((text, strong)),
    ))(source)?;
    Ok((source, Block::Paragraph { snippets }))
}
