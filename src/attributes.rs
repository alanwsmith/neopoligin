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

pub fn attributes(source: &str) -> IResult<&str, Option<Vec<Attribute>>> {
    let (source, attributes) = opt(many1(preceded(
        alt((tag("--"), tag("|"))),
        alt((id, accesskey)),
    )))(source)?;
    Ok((source, attributes))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Attribute {
    AccessKey { value: String },
    Id { value: String },
    None,
}

pub fn accesskey(source: &str) -> IResult<&str, Attribute> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("accesskey: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((
        source,
        Attribute::AccessKey {
            value: attr.to_string(),
        },
    ))
}

pub fn id(source: &str) -> IResult<&str, Attribute> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("id: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((
        source,
        Attribute::Id {
            value: attr.to_string(),
        },
    ))
}
