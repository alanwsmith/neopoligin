#![allow(unused_imports)]
use crate::attributes::accesskey_attribute::accesskey_attribute;
use crate::attributes::autocapitalize_attribute::autocapitalize_attribute;
use crate::attributes::autofocus_attribute::autofocus_attribute;
use crate::attributes::class_attribute::class_attribute;
use crate::attributes::contenteditable_attribute::contenteditable_attribute;
use crate::attributes::id_attribute::id_attribute;
use crate::attributes::type_attribute::type_attribute;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};
use nom::bytes::complete::is_not;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::sequence::terminated;

pub mod accesskey_attribute;
pub mod autocapitalize_attribute;
pub mod autofocus_attribute;
pub mod class_attribute;
pub mod contenteditable_attribute;
pub mod id_attribute;
pub mod type_attribute;

// TODO: Move away from AttributesObj
// to AttrubteList
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
#[serde(tag = "key", content = "value", rename_all = "lowercase")]
pub enum AttributeV2 {
    AccessKey(String),
    AutoCapitalize(String),
    AutoFocus,
    ContentEditable(String),
    Class(Vec<String>),
    Generic((String, String)),
    Id(String),
    Subtitle(String),
    Title(String),
    Type(String),
    Url(String),
    None,
}


pub fn attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    // dbg!(source);
    let (source, attrs) = preceded(
        alt((tag("--"), tag("|"))),
        alt((
            accesskey_attribute,
            autocapitalize_attribute,
            autofocus_attribute,
            class_attribute,
            contenteditable_attribute,
            id_attribute,
            subtitle_attribute,
            title_attribute,
            type_attribute,
            url_attribute,
            // generic must be last
            generic_attribute,
        )),
    )(source)?;
    Ok((source, attrs))
}


pub fn generic_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, key) = terminated(is_not(":"), tag(":"))(source)?;
    let (source, _) = space0(source)?;
    let (source, value) = is_not("|>\n")(source)?;
    let (source, _) = space0(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Generic((key.to_string(), value.to_string()))))
}




pub fn subtitle_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("subtitle: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Subtitle(attr.to_string())))
}

pub fn title_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("title: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Title(attr.to_string())))
}

pub fn url_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("url: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Url(attr.to_string())))
}