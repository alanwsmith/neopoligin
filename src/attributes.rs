#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
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


pub fn attributes(source: &str) -> IResult<&str, AttributesObj, VerboseError<&str>> {
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



// pub fn attributes(source: &str) -> IResult<&str, Option<AttributesObj>, VerboseError<&str>> {
//     let mut attr_obj = AttributesObj::new();
//     dbg!("-----------------------");
//     dbg!(&source);
//     let (source, attributes) = opt(many1(id_attribute))(source)?;
//     dbg!(&source);
//     dbg!("======================");
//     Ok((source, attributes))
// }

// pub fn id_attribute(source: &str) -> IResult<&str, Attribute, VerboseError<&str>> {
//     let (source, _) = tag("-- id:")(source)?;
//     dbg!(&source);
//     let (source, _) = space1(source)?;
//     let (source, value) = not_line_ending(source)?;
//     let (source, _) = line_ending(source)?;
//     Ok((source, Attribute::Id(value.to_string())))
// }




pub fn accesskey(source: &str) -> IResult<&str, Attribute, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("accesskey: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((
        source,
        Attribute::AccessKey (attr.to_string())
    ))
}


pub fn autocapitalize(source: &str) -> IResult<&str, Attribute, VerboseError<&str>> {
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

pub fn autofocus(source: &str) -> IResult<&str, Attribute, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = tag_no_case("autofocus")(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, Attribute::AutoFocus))
}

pub fn class(source: &str) -> IResult<&str, Attribute, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = tag("class:")(source)?;
    let (source, value) = many1(preceded(
        space0,
        is_not(" |>\n").map(|c: &str| c.to_string()),
    ))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, Attribute::Class(value)))
}

pub fn contenteditable(source: &str) -> IResult<&str, Attribute, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, value) = preceded(tag("contenteditable: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((
        source,
        Attribute::ContentEditable ( value.to_string())
    ))
}

pub fn id(source: &str) -> IResult<&str, Attribute, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("id: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((
        source,
        Attribute::Id (attr.to_string(),
    )
    ))
}

