use crate::attributes::accesskey_attribute::accesskey_attribute;
use crate::attributes::autocapitalize_attribute::autocapitalize_attribute;
use crate::attributes::autofocus_attribute::autofocus_attribute;
use crate::attributes::class_attribute::class_attribute;
use crate::attributes::contenteditable_attribute::contenteditable_attribute;
use crate::attributes::id_attribute::id_attribute;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use serde::{Deserialize, Serialize};

pub mod accesskey_attribute;
pub mod autocapitalize_attribute;
pub mod autofocus_attribute;
pub mod class_attribute;
pub mod contenteditable_attribute;
pub mod id_attribute;

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
    Id(String),
    None,
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

pub fn attributes_v2(source: &str) -> IResult<&str, Option<Vec<AttributeV2>>, VerboseError<&str>> {
    let (source, attrs) = opt(many1(preceded(
        alt((tag("--"), tag("|"))),
        alt((
            accesskey_attribute,
            autocapitalize_attribute,
            autofocus_attribute,
            class_attribute,
            contenteditable_attribute,
            id_attribute,
        )),
    )))(source)?;
    Ok((source, attrs))
}

pub fn attributes(source: &str) -> IResult<&str, AttributesObj, VerboseError<&str>> {
    let mut attr_obj = AttributesObj::new();
    let (source, attrs) = opt(many1(preceded(
        alt((tag("--"), tag("|"))),
        alt((
            accesskey_attribute,
            autocapitalize_attribute,
            autofocus_attribute,
            class_attribute,
            contenteditable_attribute,
            id_attribute,
        )),
    )))(source)?;

    // if let Some(d) = attrs {
    //     d.into_iter().for_each(|item| match item {
    //         Attribute::AccessKey(v) => {
    //             attr_obj.accesskey = Some(v);
    //         }
    //         Attribute::AutoCapitalize(v) => {
    //             attr_obj.autocapitalize = Some(v);
    //         }
    //         Attribute::AutoFocus => {
    //             attr_obj.autofocus = Some(true);
    //         }
    //         Attribute::Class(v) => {
    //             attr_obj.class = Some(v);
    //         }
    //         Attribute::ContentEditable(v) => {
    //             attr_obj.contenteditable = Some(v);
    //         }
    //         Attribute::Id(v) => {
    //             attr_obj.id = Some(v);
    //         }
    //         _ => (),
    //     })
    // }

    Ok((source, attr_obj))
}
