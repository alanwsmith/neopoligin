use crate::section_attrs::class::class;
use crate::section_attrs::id::id;
use nom::branch::alt;
use nom::multi::many0;
use nom::IResult;
use serde::{Deserialize, Serialize};

pub mod class;
pub mod id;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum SecAttr {
    Class(Vec<String>),
    Id(String),
    KeyValue(String, String),
    None,
}

// TODO: Switch everything over to using the individual
// class calls in SecAttrForNewClass. Then remove the
// Vec Based one in `SecAttr` and switch the new
// string based one in

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum SecAttrForNewClass {
    Class(String),
}

pub fn sec_attrs(source: &str) -> IResult<&str, Vec<SecAttr>> {
    let (source, attrs) = many0(alt((class, id)))(source.trim())?;
    Ok((source, attrs))
}
