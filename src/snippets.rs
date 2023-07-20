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



pub fn strong(source: &str) -> IResult<&str, Snippet> {
    let (source, _) = tag_no_case("<<strong|")(source)?;
    let (source, content) = is_not("|>")(source)?;
    let (source, attributes) = attributes(source)?;
    let (source, _) = tag(">>")(source)?;
    Ok((
        source,
        Snippet::Strong {
            string: content.to_string(),
            attributes,
        },
    ))
}

pub fn text(source: &str) -> IResult<&str, Snippet> {
    let (source, content) = is_not("\n<")(source)?;
    Ok((
        source,
        Snippet::Text {
            string: content.to_string(),
        },
    ))
}