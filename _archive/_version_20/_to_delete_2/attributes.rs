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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Attribute {
    AccessKey(String),
    AutoCapitalize(String),
    AutoFocus,
    ContentEditable(String),
    Class(Vec<String>),
    Id(String),
    None,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttributesObj {
    accesskey: Option<String>,
    autocapitalize: Option<String>,
    autofocus: Option<bool>,
    contenteditable: Option<String>,
    class: Option<Vec<String>>,
    id: Option<String>,
}

impl AttributesObj {
    pub fn new() -> AttributesObj {
        AttributesObj { 
            accesskey: None, 
            autocapitalize: None,
            autofocus: None,
            contenteditable: None,
            class: None,
            id: None, 
        }
    }
}


pub fn attributes(source: &str) -> IResult<&str, AttributesObj> {
    let mut attr_obj = AttributesObj::new();
    let (source, attrs) = opt(many1(preceded(
        alt((tag("--"), tag("|"))),
        alt((
            accesskey,
            autocapitalize,
            autofocus,
            class,
            contenteditable,
            id,
        )),
    )))(source)?;

    if let Some(d) = attrs {
        d.into_iter().for_each(|item| {
            match item {
                Attribute::AccessKey(v) => { attr_obj.accesskey = Some(v); }
                Attribute::AutoCapitalize(v) => { attr_obj.autocapitalize = Some(v); }
                Attribute::AutoFocus => { attr_obj.autofocus = Some(true); }
                Attribute::Class(v) => { attr_obj.class = Some(v); }
                Attribute::ContentEditable(v) => { attr_obj.contenteditable = Some(v); }
                Attribute::Id(v) => { attr_obj.id = Some(v); }
                _ => () 
            }
        })
    } 

    Ok((source, attr_obj))
}


pub fn attributes_old(source: &str) -> IResult<&str, Option<Vec<Attribute>>> {
    let (source, attributes) = opt(many1(preceded(
        alt((tag("--"), tag("|"))),
        alt((
            id,
            accesskey,
            autocapitalize,
            autofocus,
            class,
            contenteditable,
        )),
    )))(source)?;
    Ok((source, attributes))
}


pub fn accesskey(source: &str) -> IResult<&str, Attribute> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("accesskey: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((
        source,
        Attribute::AccessKey (attr.to_string())
    ))
}


pub fn autocapitalize(source: &str) -> IResult<&str, Attribute> {
    // autocapitzlize has a specific set of options but the
    // ROI of limiting to them isn't high for phase 1 so
    // just passing in whatever string. Something to look into
    // for a future iteration maybe
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("autocapitalize: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((
        source,
        Attribute::AutoCapitalize (attr.to_string())
    ))
}

pub fn autofocus(source: &str) -> IResult<&str, Attribute> {
    let (source, _) = space0(source)?;
    let (source, attr) = tag_no_case("autofocus")(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, Attribute::AutoFocus))
}

pub fn class(source: &str) -> IResult<&str, Attribute> {
    let (source, _) = space0(source)?;
    let (source, attr) = tag("class:")(source)?;
    let (source, value) = many1(preceded(
        space0,
        is_not(" |>\n").map(|c: &str| c.to_string()),
    ))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, Attribute::Class(value)))
}

pub fn contenteditable(source: &str) -> IResult<&str, Attribute> {
    let (source, _) = space0(source)?;
    let (source, value) = preceded(tag("contenteditable: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((
        source,
        Attribute::ContentEditable ( value.to_string())
    ))
}

pub fn id(source: &str) -> IResult<&str, Attribute> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("id: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((
        source,
        Attribute::Id (attr.to_string(),
    )
    ))
}
