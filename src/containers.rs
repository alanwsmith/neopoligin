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
use crate::blocks::Block;
use crate::blocks::paragraph;



#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Container {
    ListItem { content: Vec<Block> },
    None,
}

pub fn list_item(source: &str) -> IResult<&str, Container> {
    let (source, content) = many1(preceded(tag("- "), paragraph))(source)?;
    Ok((source, Container::ListItem { content }))
}
